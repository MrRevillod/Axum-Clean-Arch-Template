use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::features::user::domain::entity::User;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct UserModel {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub validated: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<UserModel> for User {
    fn from(user_model: UserModel) -> Self {
        User {
            id: user_model.id,
            username: user_model.username,
            email: user_model.email,
            password: user_model.password,
            validated: user_model.validated,
            created_at: user_model.created_at,
            updated_at: user_model.updated_at,
        }
    }
}

impl From<User> for UserModel {
    fn from(user: User) -> Self {
        UserModel {
            id: user.id,
            username: user.username,
            email: user.email,
            password: user.password,
            validated: user.validated,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
