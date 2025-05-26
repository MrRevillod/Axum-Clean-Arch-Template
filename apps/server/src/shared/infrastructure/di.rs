use axum::extract::FromRef;
use shaku::module;
use std::sync::Arc;

use crate::{
    features::user::{
        application::services::password::BcryptPasswordHasher,
        application::usecases::{
            create::CreateUserCaseImpl, delete::DeleteUserCaseImpl,
            get::GetUsersCaseImpl, update::UpdateUserCaseImpl,
        },
        infrastructure::repository::PostgresUserRepository,
    },
    shared::infrastructure::{cache::RedisCache, database::PostgresDatabase},
};

pub type Inject<T> = shaku_axum::Inject<AppModule, T>;

#[derive(Clone)]
pub struct AppState {
    pub module: Arc<AppModule>,
}

impl FromRef<AppState> for Arc<AppModule> {
    fn from_ref(app_state: &AppState) -> Arc<AppModule> {
        app_state.module.clone()
    }
}

module! {

    pub AppModule {
        components = [
            PostgresDatabase,
            PostgresUserRepository,

            RedisCache,

            BcryptPasswordHasher,

            GetUsersCaseImpl,
            CreateUserCaseImpl,
            UpdateUserCaseImpl,
            DeleteUserCaseImpl
        ],
        providers = []
    }
}
