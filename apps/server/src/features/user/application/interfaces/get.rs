// This module defines the GetUsersCase Trait/Interface and its
// corresponding Input format and return type.

use async_trait::async_trait;
use shaku::Interface;

use crate::features::user::domain::{
    entity::{PaginatedData, User},
    errors::UserError,
};

// TODO!: impl of optional filters

pub struct UserQueryInput {
    pub page: i64,
    pub page_size: i64,
}

// The implementation of the GetUsersCase trait
// is in: /features/user/application/use_cases/get.rs

#[async_trait]
pub trait GetUsersCase: Interface {
    async fn execute(
        &self,
        query: UserQueryInput,
    ) -> Result<PaginatedData<User>, UserError>;
}
