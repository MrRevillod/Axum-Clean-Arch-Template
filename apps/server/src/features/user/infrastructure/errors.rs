use axum::http::StatusCode;
use axum_responses::HttpResponse;
use serde_json::json;

use crate::features::user::domain::errors::UserError;

impl From<UserError> for HttpResponse {
    fn from(value: UserError) -> Self {
        match value {
            UserError::UsernameAlreadyExists => HttpResponse {
                status: StatusCode::CONFLICT,
                body: json!({
                    "field": "username",
                    "message": "Username already exists",
                }),
            },
            UserError::EmailAlreadyExists => HttpResponse {
                status: StatusCode::CONFLICT,
                body: json!({
                    "field": "email",
                    "message": "Email already exists",
                }),
            },
            UserError::NotFound => HttpResponse {
                status: StatusCode::NOT_FOUND,
                body: json!({
                    "message": "User not found",
                }),
            },
            UserError::UnexpectedError => HttpResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                body: json!({
                    "message": "Unexpected error",
                }),
            },
            UserError::InvalidEmail => HttpResponse {
                status: StatusCode::BAD_REQUEST,
                body: json!({
                    "field": "email",
                    "message": "The provided email is not valid to register",
                }),
            },
            UserError::InvalidId => HttpResponse {
                status: StatusCode::BAD_REQUEST,
                body: json!({
                    "field": "id",
                    "message": "The provided id is not valid",
                }),
            },
        }
    }
}
