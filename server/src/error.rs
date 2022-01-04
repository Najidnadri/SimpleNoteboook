use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterError {
    pub username_existed: bool,
    pub email_taken: bool,
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