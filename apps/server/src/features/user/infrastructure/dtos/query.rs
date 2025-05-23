use serde::Deserialize;
use validator::Validate;

use crate::features::user::application::interfaces::get::UserQueryInput;

#[derive(Debug, Deserialize, Validate)]
pub struct UserQuery {
    #[validate(range(min = 1))]
    pub page: Option<i64>,
    #[validate(range(min = 5))]
    #[serde(rename = "pageSize")]
    pub page_size: Option<i64>,
}

impl From<UserQuery> for UserQueryInput {
    fn from(value: UserQuery) -> Self {
        Self {
            page: value.page.unwrap_or(0),
            page_size: value.page_size.unwrap_or(25),
        }
    }
}
