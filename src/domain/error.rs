use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DomainError {
    ApiError(String),
    DatabaseError(String),
    OtherError(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DomainError::ApiError(ref message) => write!(f, "API error: {}", message),
            DomainError::DatabaseError(ref message) => write!(f, "Database error: {}", message),
            DomainError::OtherError(ref message) => write!(f, "Other error: {}", message),
        }
    }
}

impl Error for DomainError {}