use axum::extract::Path;
use axum_responses::{response, ControllerResult};

use crate::{
    features::user::{
        application::interfaces::{
            create::CreateUserCase, delete::DeleteUserCase, get::GetUsersCase,
            update::UpdateUserCase,
        },
        infrastructure::dtos::body::{
            CreateUserDto, PaginatedDataDTO, UpdateUserDto,
        },
    },
    shared::infrastructure::{
        di::Inject,
        http::extractors::{BodyValidator, QueryValidator},
    },
};

use super::dtos::query::UserQuery;

pub async fn get_users(
    use_case: Inject<dyn GetUsersCase>,
    QueryValidator(query): QueryValidator<UserQuery>,
) -> ControllerResult {
    let paginated_data = use_case.execute(query.into()).await?;
    let paginated_data_dto = PaginatedDataDTO::from(paginated_data);

    response!(200, { paginated_data_dto })
}

pub async fn create_user(
    use_case: Inject<dyn CreateUserCase>,
    BodyValidator(user_data): BodyValidator<CreateUserDto>,
) -> ControllerResult {
    use_case.execute(user_data.into()).await?;
    response!(201, { "message": "User created" })
}

pub async fn update_user(
    use_case: Inject<dyn UpdateUserCase>,
    Path(id): Path<String>,
    BodyValidator(user_data): BodyValidator<UpdateUserDto>,
) -> ControllerResult {
    use_case.execute(id, user_data.into()).await?;
    response!(200, { "message": "User updated" })
}

pub async fn delete_user(
    use_case: Inject<dyn DeleteUserCase>,
    Path(id): Path<String>,
) -> ControllerResult {
    use_case.execute(id).await?;
    response!(200, { "message": "User deleted" })
}
