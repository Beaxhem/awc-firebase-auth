use std::fmt;
use serde::{Serialize, Deserialize};

pub enum LoginError {
    EmailNotFound,
    InvalidPassword,
    UserDisabled,
    EmailExists,
    OperationNotAllowed,
    TooManyAttempts,
    Unknown
}

pub enum RegisterError {
    EmailExists,
    OperationNotAllowed,
    TooManyAttempts,
    Unknown
}


impl fmt::Display for LoginError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LoginError::EmailNotFound => write!(f, "Email not found"),
            LoginError::InvalidPassword => write!(f, "Invalid password"),
            LoginError::UserDisabled => write!(f, "User disabled"),
            LoginError::EmailExists => write!(f, "Email exists"),
            LoginError::OperationNotAllowed => write!(f, "Operation not allowed"),        
            LoginError::TooManyAttempts => write!(f, "Too many attempts"),
            LoginError::Unknown => write!(f, "Unknown"),
        }
    }
}

impl fmt::Display for RegisterError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterError::EmailExists => write!(f, "Email exists"),
            RegisterError::OperationNotAllowed => write!(f, "Operation not allowed"),
            RegisterError::TooManyAttempts => write!(f, "Too many attempts"),
            RegisterError::Unknown => write!(f, "Unknown"),
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
            "EMAIL_EXISTS" => LoginError::EmailExists,
            "OPERATION_NOT_ALLOWED" => LoginError::OperationNotAllowed,
            "TOO_MANY_ATTEMPTS_TRY_LATER" => LoginError::TooManyAttempts,
            _ => LoginError::Unknown,
        }
    }
}