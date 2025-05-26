use crate::shared::domain::cache::CacheError;

#[derive(Debug)]
pub enum UserError {
    NotFound,
    EmailAlreadyExists,
    UsernameAlreadyExists,
    UnexpectedError,
    InvalidEmail,
    InvalidId,
}

impl From<CacheError> for UserError {
    fn from(value: CacheError) -> Self {
        dbg!(value);

        UserError::UnexpectedError
    }
}
