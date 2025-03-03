use crate::domain::TextExtractor;
use crate::infrastructure::error::AppError;
use std::error::Error as StdError;
use std::path::Path;
use std::fs;

pub struct PlainTextExtractor;

impl TextExtractor for PlainTextExtractor {
    fn supports(&self, file_type: &str) -> bool {
        let file_type = file_type.to_lowercase();
        matches!(file_type.as_str(), "txt" | "md" | "csv" | "json" | "xml" | "html" | "log")
    }
    
    fn extract(&self, file_path: &Path) -> Result<String, Box<dyn StdError + Send + Sync>> {
        fs::read_to_string(file_path)
            .map_err(|e| AppError::IoError(e).into())
    }
    
}