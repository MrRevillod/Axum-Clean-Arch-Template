use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;

use crate::{
    features::user::{
        application::interfaces::get::{GetUsersCase, UserQueryInput},
        domain::{
            entity::{PaginatedData, User},
            errors::UserError,
            repository::UserRepository,
        },
    },
    shared::domain::cache::Cache,
};

#[derive(Component)]
#[shaku(interface = GetUsersCase)]
pub struct GetUsersCaseImpl {
    #[shaku(inject)]
    repository: Arc<dyn UserRepository>,
    #[shaku(inject)]
    cache: Arc<dyn Cache>,
}

#[async_trait]
impl GetUsersCase for GetUsersCaseImpl {
    async fn execute(
        &self,
        query: UserQueryInput,
    ) -> Result<PaginatedData<User>, UserError> {
        let cache_key = format!("users:{}:{}", query.page, query.page_size);

        let data = self.cache.get_json(&cache_key).await?;

        self.repository.find_all(query.page, query.page_size).await
    }
}
