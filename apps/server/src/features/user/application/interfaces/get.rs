use async_trait::async_trait;
use shaku::Interface;

use crate::features::user::domain::{entity::User, errors::UserError};

// todo!: impl of optional filters

#[async_trait]
pub trait GetUsersCase: Interface {
    async fn execute(&self) -> Result<Vec<User>, UserError>;
}
