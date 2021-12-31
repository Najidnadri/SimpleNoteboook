pub enum RegisterError {
    UsernameExisted(String),
    EmailTaken(String),
    PasswordInsecure(String),
}

pub enum LoginError {
    WrongPassword(String),
    UsernameErr(String),
}

pub enum AppError {
    LoginError(LoginError),
    RegisterError(RegisterError,)
}