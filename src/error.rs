use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub enum LoginError {
    EmailNotFound,
    InvalidPassword,
    UserDisabled,
    OperationNotAllowed,
    TooManyAttempts,
    Unknown,
}

impl fmt::Display for LoginError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LoginError::EmailNotFound => write!(f, "Email not found"),
            LoginError::InvalidPassword => write!(f, "Invalid password"),
            LoginError::UserDisabled => write!(f, "User disabled"),
            LoginError::OperationNotAllowed => write!(f, "Operation not allowed"),
            LoginError::TooManyAttempts => write!(f, "Too many attempts"),
            LoginError::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug)]
pub enum RegisterError {
    EmailExists,
    OperationNotAllowed,
    TooManyAttempts,
    Unknown,
    MissingPassword,
}

impl fmt::Display for RegisterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterError::EmailExists => write!(f, "Email exists"),
            RegisterError::OperationNotAllowed => write!(f, "Operation not allowed"),
            RegisterError::TooManyAttempts => write!(f, "Too many attempts"),
            RegisterError::Unknown => write!(f, "Unknown"),
            RegisterError::MissingPassword => write!(f, "Missing password"),
        }
    }
}

#[derive(Debug)]
pub enum AccountError {
    InvalidIdToken,
    UserNotFound,
    Unknown,
}

impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AccountError::InvalidIdToken => write!(f, "Invalid Id token"),
            AccountError::UserNotFound => write!(f, "User not found"),
            AccountError::Unknown => write!(f, "Unknown error"),
        }
    }
}

#[derive(Debug)]
pub enum RefreshTokenError {
    TokenExpired,
    UserDisabled,
    UserNotFound,
    InvalidRefreshToken,
    InvalidGrantType,
    Unknown,
}

impl fmt::Display for RefreshTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RefreshTokenError::TokenExpired => write!(f, "TOKEN_EXPIRED"),
            RefreshTokenError::UserDisabled => write!(f, "USER_DISABLED"),
            RefreshTokenError::UserNotFound => write!(f, "USER_NOT_FOUND"),
            RefreshTokenError::InvalidRefreshToken => write!(f, "INVALID_REFRESH_TOKEN"),
            RefreshTokenError::InvalidGrantType => write!(f, "INVALID_GRANT_TYPE"),
            RefreshTokenError::Unknown => write!(f, "Unknown error"),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct ErrorBody {
    domain: String,
    reason: String,
    message: String,
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    errors: Vec<ErrorBody>,
    code: i32,
    message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorContainer {
    pub error: Error,
}

impl Error {
    pub fn register_error(&self) -> RegisterError {
        match self.message.as_str() {
            "EMAIL_EXISTS" => RegisterError::EmailExists,
            "OPERATION_NOT_ALLOWED" => RegisterError::OperationNotAllowed,
            "TOO_MANY_ATTEMPTS_TRY_LATER" => RegisterError::TooManyAttempts,
            _ => RegisterError::Unknown,
        }
    }

    pub fn login_error(&self) -> LoginError {
        match self.message.as_str() {
            "EMAIL_NOT_FOUND" => LoginError::EmailNotFound,
            "INVALID_PASSWORD" => LoginError::InvalidPassword,
            "USER_DISABLED" => LoginError::UserDisabled,
            "OPERATION_NOT_ALLOWED" => LoginError::OperationNotAllowed,
            "TOO_MANY_ATTEMPTS_TRY_LATER" => LoginError::TooManyAttempts,
            _ => LoginError::Unknown,
        }
    }

    pub fn account_error(&self) -> AccountError {
        match self.message.as_str() {
            "INVALID_ID_TOKEN" => AccountError::InvalidIdToken,
            "USER_NOT_FOUND" => AccountError::UserNotFound,
            _ => AccountError::Unknown,
        }
    }

    pub fn refresh_token_error(&self) -> RefreshTokenError {
        match self.message.as_str() {
            "TOKEN_EXPIRED" => RefreshTokenError::InvalidGrantType,
            "USER_DISABLED" => RefreshTokenError::UserDisabled,
            "USER_NOT_FOUND" => RefreshTokenError::UserNotFound,
            "INVALID_REFRESH_TOKEN" => RefreshTokenError::InvalidRefreshToken,
            "INVALID_GRANT_TYPE" => RefreshTokenError::InvalidGrantType,
            _ => RefreshTokenError::Unknown,
        }
    }
}
