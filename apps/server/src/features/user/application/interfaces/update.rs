use async_trait::async_trait;
use shaku::Interface;

use crate::features::user::domain::{entity::User, errors::UserError};

pub struct UpdateUserInput {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[async_trait]
pub trait UpdateUserCase: Interface {
    async fn execute(
        &self,
        id: String,
        input: UpdateUserInput,
    ) -> Result<User, UserError>;
}
