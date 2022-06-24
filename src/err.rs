use std::error::Error;
use std::fmt::Display;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
pub struct AppError {
    error_type: ErrorType,
    source: Box<dyn Error + 'static>,
}

#[derive(Debug)]
pub enum ErrorType {
    NetworkError,
    FileIOError,
    UnexpectedError,
}

impl AppError {}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.source.as_ref())
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.error_type {
            ErrorType::FileIOError => write!(f, "File IO error: {}", self.source),
            ErrorType::NetworkError => write!(f, "Network error: {}", self.source),
            ErrorType::UnexpectedError => write!(f, "Unexpected faiure {}", self.source),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(source: std::io::Error) -> Self {
        Self {
            error_type: ErrorType::FileIOError,
            source: Box::new(source),
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        Self {
            error_type: ErrorType::NetworkError,
            source: Box::new(err),
        }
    }
}
