use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;

use crate::features::user::{
    application::interfaces::get::GetUsersCase,
    domain::{entity::User, errors::UserError, repository::UserRepository},
};

#[derive(Component)]
#[shaku(interface = GetUsersCase)]
pub struct GetUsersCaseImpl {
    #[shaku(inject)]
    repository: Arc<dyn UserRepository>,
}

#[async_trait]
impl GetUsersCase for GetUsersCaseImpl {
    async fn execute(&self) -> Result<Vec<User>, UserError> {
        self.repository.find_all().await
    }
}
