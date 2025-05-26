use std::ops::Deref;

use axum::{
    extract::{FromRequest, Json, Query, Request},
    http::StatusCode,
};

use axum_responses::HttpResponse;
use serde_json::json;
use validator::Validate;

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

#[derive(Debug)]
pub struct QueryValidator<T>(pub T);

impl<T> Deref for QueryValidator<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S, T> FromRequest<S> for QueryValidator<T>
where
    S: Send + Sync,
    T: Validate + for<'de> serde::Deserialize<'de> + Send,
{
    type Rejection = HttpResponse;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Query(value) =
            Query::<T>::from_request(req, state).await.map_err(|_| {
                HttpResponse {
                    status: StatusCode::BAD_REQUEST,
                    body: json!({ "message": "Invalid query format" }),
                }
            })?;

        value.validate().map_err(|err| HttpResponse {
            status: StatusCode::BAD_REQUEST,
            body: json!({
                "message": "Invalid query format",
                "errors": err.to_string()
            }),
        })?;

        Ok(Self(value))
    }
}
