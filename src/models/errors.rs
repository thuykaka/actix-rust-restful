#[derive(thiserror::Error, Debug)]
#[error("...")]
#[allow(dead_code)]
pub enum Error {
    #[error("sea_orm::DbErr: {0}")]
    Db(#[from] sea_orm::DbErr),

    #[error("ENV VARIABLE for `{0}` is not set")]
    EnvironmentVariableNotSet(String),

    #[error("Resource Not Found: {0}")]
    NotFound(String),

    #[error("{0}")]
    ServerStartFailed(#[from] std::io::Error),

    #[error("InvalidId: ID {0} is not valid")]
    InvalidId(String),

    #[error("Email already exists")]
    EmailAlreadyExists,

    #[error("Invalid email format")]
    InvalidEmail,

    #[error("Password too short")]
    PasswordTooShort,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Failed to generate authentication token")]
    FailedToGenerateToken,

    #[error("Failed to verify authentication token")]
    FailedToVerifyToken,

    #[error("Wrong email or password")]
    WrongEmailOrPassword,

    #[error("User not found")]
    UserNotFound,
}
