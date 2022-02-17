use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct AppError {
    message: String,
}

impl AppError {
    pub fn new<T: ToString>(message: T) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl std::cmp::PartialEq for AppError {
    fn eq(&self, _other: &AppError) -> bool {
        false
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        &self.message
    }
}

pub type AppResult<T> = Result<T, AppError>;
