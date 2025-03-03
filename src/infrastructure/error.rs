use std::error::Error as StdError;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum AppError {
    IoError(io::Error),
    ApiError(String),
    PdfExtractError(String),
    DocxExtractError(String),
    JsonError(serde_json::Error),
    #[allow(dead_code)]
    NotImplemented,
    Other(Box<dyn StdError + Send + Sync>),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::IoError(e) => write!(f, "IO Error: {}", e),
            AppError::ApiError(msg) => write!(f, "API Error: {}", msg),
            AppError::PdfExtractError(msg) => write!(f, "PDF Extract Error: {}", msg),
            AppError::DocxExtractError(msg) => write!(f, "DOCX Extract Error: {}", msg),
            AppError::JsonError(e) => write!(f, "JSON Error: {}", e),
            AppError::NotImplemented => write!(f, "Feature not implemented"),
            AppError::Other(e) => write!(f, "Error: {}", e),
        }
    }
}

impl StdError for AppError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            AppError::IoError(e) => Some(e),
            AppError::JsonError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::IoError(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::JsonError(err)
    }
}

impl From<Box<dyn StdError + Send + Sync>> for AppError {
    fn from(err: Box<dyn StdError + Send + Sync>) -> Self {
        AppError::Other(err)
    }
}