use axum::extract::Path;
use axum_responses::{response, ControllerResult};

use crate::features::user::{
    application::{
        interfaces::{
            create::CreateUserCase, delete::DeleteUserCase, get::GetUsersCase,
            update::UpdateUserCase,
        },
        usecases::{
            create::CreateUserCaseImpl, delete::DeleteUserCaseImpl,
            get::GetUsersCaseImpl, update::UpdateUserCaseImpl,
        },
    },
    infrastructure::{
        dtos::body::{CreateUserDto, UpdateUserDto},
        models::UserModel,
    },
};

use crate::infrastructure::{di::InjectUseCase, http::extractors::BodyValidator};

pub async fn get_users(
    use_case: InjectUseCase<dyn GetUsersCase, GetUsersCaseImpl>,
) -> ControllerResult {
    let users = use_case.execute().await?;

    // The users are returned as a vector of `User` instances.
    // We need to convert them into `UserModel` instances for the response.

    let data = users
        .into_iter()
        .map(|user| UserModel::from(user))
        .collect::<Vec<UserModel>>();

    response!(200, { "data": data })
}

pub async fn create_user(
    use_case: InjectUseCase<dyn CreateUserCase, CreateUserCaseImpl>,
    BodyValidator(user_data): BodyValidator<CreateUserDto>,
) -> ControllerResult {
    use_case.execute(user_data.into()).await?;
    response!(201, { "message": "User created" })
}

pub async fn update_user(
    use_case: InjectUseCase<dyn UpdateUserCase, UpdateUserCaseImpl>,
    Path(id): Path<String>,
    BodyValidator(user_data): BodyValidator<UpdateUserDto>,
) -> ControllerResult {
    use_case.execute(id, user_data.into()).await?;
    response!(200, { "message": "User updated" })
}

pub async fn delete_user(
    use_case: InjectUseCase<dyn DeleteUserCase, DeleteUserCaseImpl>,
    Path(id): Path<String>,
) -> ControllerResult {
    use_case.execute(id).await?;
    response!(200, { "message": "User deleted" })
}
