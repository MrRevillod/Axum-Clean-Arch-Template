// This module contains the data transfer objects (DTOs) for user creation and update.

// |----------------------------------------------------------------|
// |                 Input entities between layers                  |
// |----------------------------------------------------------------|
// | User Infrastructure Layer (CreateUserDto) |     Controller     |
// |----------------------------------------------------------------|
// | User Application Layer (CreateUserInput)  |      Use Case      |
// |----------------------------------------------------------------|
// |         User Domain Layer (User)          |     Repository     |
// |----------------------------------------------------------------|

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::features::user::{
    application::interfaces::{create::CreateUserInput, update::UpdateUserInput},
    domain::entity::{PaginatedData, User},
    infrastructure::models::UserModel,
};

use super::validators::{
    password_schema, validate_optional_password_pairs, validate_password_pairs,
};

// The DTOs are used to validate the incoming request data
// and to convert the data into the appropriate input types for the use cases.

// The `#[validate]` attribute is used to specify the validation rules for each field

// The `#[serde(rename = "confirmPassword")]` attribute is used to rename the field
// in the JSON request body from `confirmPassword` to `confirm_password`
// This is necessary bc the field name in the JSON request body
// does not match the field name in the struct

#[derive(Deserialize, Validate)]
#[validate(schema(function = "validate_password_pairs"))]
pub struct CreateUserDto {
    #[validate(length(min = 5, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(custom(function = "password_schema"))]
    pub password: String,
    #[validate(custom(function = "password_schema"))]
    #[serde(rename = "confirmPassword")]
    pub confirm_password: String,
}

// This trait implementation converts the `CreateUserDto` into the `CreateUserInput`
// This is necessary bc the `CreateUserInput` type is used in the use case
// and the `CreateUserDto` type is used in the controller

// | Controller (CreateUserDto) -> Use Case (CreateUserInput) |

impl From<CreateUserDto> for CreateUserInput {
    fn from(dto: CreateUserDto) -> Self {
        CreateUserInput {
            username: dto.username,
            email: dto.email,
            password: dto.password,
        }
    }
}

#[derive(Deserialize, Validate)]
#[validate(schema(function = "validate_optional_password_pairs"))]
pub struct UpdateUserDto {
    #[validate(length(min = 5, max = 50))]
    pub username: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(custom(function = "password_schema"))]
    pub password: Option<String>,
    #[validate(custom(function = "password_schema"))]
    #[serde(rename = "confirmPassword")]
    pub confirm_password: Option<String>,
}

// This trait implementation converts the `UpdateUserDto` into the `UpdateUserInput`
// This is necessary bc the `UpdateUserInput` type is used in the use case
// and the `UpdateUserDto` type is used in the controller

// | Controller (UpdateUserDto) -> Use Case (UpdateUserInput) |

impl From<UpdateUserDto> for UpdateUserInput {
    fn from(dto: UpdateUserDto) -> Self {
        UpdateUserInput {
            username: dto.username,
            email: dto.email,
            password: dto.password,
        }
    }
}

// This structs represent the response of GetUsers Controller
// Its a DTO that is used to convert the data from the use case
// to the response format that is expected by the client

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedDataDTO {
    pub data: Vec<UserModel>,
    pub count: i64,
    pub total_pages: i64,
    pub page: i64,
    pub page_size: i64,
}

impl From<PaginatedData<User>> for PaginatedDataDTO {
    fn from(paginated_data: PaginatedData<User>) -> Self {
        PaginatedDataDTO {
            data: paginated_data
                .data
                .into_iter()
                .map(|user| UserModel::from(user))
                .collect(),
            count: paginated_data.count,
            total_pages: paginated_data.total_pages,
            page: paginated_data.page,
            page_size: paginated_data.page_size,
        }
    }
}
