use async_trait::async_trait;
use shaku::Interface;

use crate::features::user::domain::errors::UserError;

#[async_trait]
pub trait DeleteUserCase: Interface {
    async fn execute(&self, user_id: String) -> Result<(), UserError>;
}
