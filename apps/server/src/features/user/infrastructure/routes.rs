use axum::routing::{delete, get, post, Router};

use super::controllers::*;
use crate::shared::infrastructure::di::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/users", get(get_users))
        .route("/users/", post(create_user))
        .route("/users/{id}", post(update_user))
        .route("/users/{id}", delete(delete_user))
        .with_state(state)
}
