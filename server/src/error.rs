pub enum RegisterError {
    UsernameExisted(String),
    EmailTaken(String),
    PasswordInsecure(String),
}