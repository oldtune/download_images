use std::error::Error;
use std::fmt::Debug as DebugTrait;
use std::fmt::Display;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    UnexpectedError(Box<dyn Error + Send>),
    FileNotFound(&'static str),
}

impl AppError {}

impl Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::FileNotFound(file) => write!(f, "File {} is not found", file),
            AppError::UnexpectedError(err) => write!(f, "Unexpected error: {:?}", err.as_ref()),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::UnexpectedError(Box::new(err))
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        Self::UnexpectedError(Box::new(err))
    }
}

impl From<log::SetLoggerError> for AppError {
    fn from(err: log::SetLoggerError) -> Self {
        Self::UnexpectedError(Box::new(err))
    }
}
