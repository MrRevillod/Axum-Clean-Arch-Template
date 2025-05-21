use axum::{
    extract::{FromRef, FromRequest, FromRequestParts, Json, Request},
    http::{request::Parts, StatusCode},
};

use serde_json::json;
use validator::Validate;

use axum_responses::HttpResponse;

// ----------------------------------------------------------------------------

pub struct BodyValidator<T>(pub T);

impl<S, T> FromRequest<S> for BodyValidator<T>
where
    T: Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S>,
{
    type Rejection = HttpResponse;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) =
            Json::<T>::from_request(req, state)
                .await
                .map_err(|_| HttpResponse {
                    status: StatusCode::BAD_REQUEST,
                    body: json!({ "error": "Invalid request body" }),
                })?;

        data.validate().map_err(|e| HttpResponse {
            status: StatusCode::BAD_REQUEST,
            body: json!({
                "message": "Validation failed",
                "errors": e.to_string(),
            }),
        })?;

        Ok(BodyValidator(data))
    }
}

// ----------------------------------------------------------------------------

use shaku::{HasComponent, Interface, ModuleInterface};
use std::sync::Arc;
use std::{marker::PhantomData, ops::Deref};

pub struct CustomInjectExtractor<
    M: ModuleInterface + HasComponent<I> + ?Sized,
    I: Interface + ?Sized,
    Impl: Interface + ?Sized,
>(pub Arc<I>, PhantomData<M>, PhantomData<Impl>);

impl<S, M, I, Impl> FromRequestParts<S> for CustomInjectExtractor<M, I, Impl>
where
    S: Send + Sync,
    M: ModuleInterface + HasComponent<I> + ?Sized,
    I: Interface + ?Sized,
    Impl: Interface + ?Sized,
    Arc<M>: FromRef<S>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        _req: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let component = Arc::<M>::from_ref(state).resolve();
        Ok(Self(component, PhantomData, PhantomData))
    }
}

impl<M, I, Impl> Deref for CustomInjectExtractor<M, I, Impl>
where
    M: ModuleInterface + HasComponent<I> + ?Sized,
    I: Interface + ?Sized,
    Impl: Interface + ?Sized,
{
    type Target = I;

    fn deref(&self) -> &Self::Target {
        Arc::as_ref(&self.0)
    }
}
