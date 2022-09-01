use std::fmt;

use awc::error::{SendRequestError, JsonPayloadError};

use super::Provider;

pub enum CodeExhangeError {
    UnknownError,
    ParamError(&'static str),
    SendRequestError(SendRequestError),
    DecodingError(JsonPayloadError),
    UnsupportedProvider(Provider)
}

impl fmt::Display for CodeExhangeError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CodeExhangeError::UnknownError => write!(f, "Unknown error"),
            CodeExhangeError::ParamError(key) => write!(f, "URL param error: {} key is not found in info", key),
            CodeExhangeError::SendRequestError(err) => write!(f, "{}", err),
            CodeExhangeError::DecodingError(err) => write!(f, "{}", err),
            CodeExhangeError::UnsupportedProvider(provider) => write!(f, "{}", provider),
        }
    }
}