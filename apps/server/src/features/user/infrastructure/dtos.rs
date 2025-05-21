use regex::Regex;
use serde::Deserialize;
use validator::{Validate, ValidationError};

use crate::features::user::application::interfaces::{
    create::CreateUserInput, update::UpdateUserInput,
};

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

impl From<UpdateUserDto> for UpdateUserInput {
    fn from(dto: UpdateUserDto) -> Self {
        UpdateUserInput {
            username: dto.username,
            email: dto.email,
            password: dto.password,
        }
    }
}

fn validate_password_pairs(dto: &CreateUserDto) -> Result<(), ValidationError> {
    if dto.password != dto.confirm_password {
        return Err(ValidationError::new("Passwords must match"));
    }
    Ok(())
}

fn validate_optional_password_pairs(
    dto: &UpdateUserDto,
) -> Result<(), ValidationError> {
    match (&dto.password, &dto.confirm_password) {
        (Some(pwd), Some(conf)) if pwd != conf => {
            Err(ValidationError::new("Passwords must match"))
        }
        (Some(_), None) | (None, Some(_)) => Err(ValidationError::new(
            "Either provide both password fields or neither",
        )),
        _ => Ok(()),
    }
}

fn password_schema(password: &str) -> Result<(), ValidationError> {
    if password.len() < 8 || password.len() > 100 {
        return Err(ValidationError::new(
            "Password must be 8-100 characters long",
        ));
    }

    let mut has_digit = false;
    let mut has_uppercase = false;
    let mut has_lowercase = false;

    for c in password.chars() {
        match c {
            'A'..='Z' => has_uppercase = true,
            'a'..='z' => has_lowercase = true,
            '0'..='9' => has_digit = true,
            _ => {}
        }
    }

    if !has_uppercase {
        return Err(ValidationError::new(
            "Password must contain at least one uppercase letter",
        ));
    }

    if !has_lowercase {
        return Err(ValidationError::new(
            "Password must contain at least one lowercase letter",
        ));
    }

    if !has_digit {
        return Err(ValidationError::new(
            "Password must contain at least one digit",
        ));
    }

    let special_chars_regex = Regex::new(r"[@$!%*?&]").unwrap();
    if !special_chars_regex.is_match(password) {
        return Err(ValidationError::new(
            "Password must contain at least one special character (@$!%*?&)",
        ));
    }

    Ok(())
}
