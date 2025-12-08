use std::error::Error;
use std::fmt;
use uuid::Uuid;

// Define a custom error type for the application
#[derive(Debug)]
enum AppError {
    InvalidInput(String),
    DatabaseError(String),
    UnknownError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AppError::InvalidInput(ref msg) => write!(f, "Invalid input: {}", msg),
            AppError::DatabaseError(ref msg) => write!(f, "Database error: {}", msg),
            AppError::UnknownError(ref msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        match *self {
            AppError::InvalidInput(_) => "Invalid input",
            AppError::DatabaseError(_) => "Database error",
            AppError::UnknownError(_) => "Unknown error",
        }
    }
}

// Generate a random UUID
pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

// Define a function to handle errors and return a custom error message
pub fn handle_error(error: &str) -> AppError {
    AppError::UnknownError(error.to_string())
}
