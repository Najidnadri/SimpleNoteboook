use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub enum RegisterError {
    UsernameExisted(String),
    EmailTaken(String),
    PasswordInsecure(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum LoginError {
    WrongPassword(String),
    UsernameErr(String),
}

pub enum AppError {
    LoginError(LoginError),
    RegisterError(RegisterError,)
}