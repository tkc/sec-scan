use crate::domain::service::OutputService;
use crate::domain::models::ScanResult;
use crate::infrastructure::error::AppError;
use std::error::Error as StdError;
use std::path::Path;
use std::fs;

pub struct JsonOutputFormatter;

impl JsonOutputFormatter {
    pub fn new() -> Self {
        JsonOutputFormatter
    }
}

impl OutputService for JsonOutputFormatter {
    fn format_results(&self, results: &[ScanResult]) -> Result<String, Box<dyn StdError + Send + Sync>> {
        serde_json::to_string_pretty(results)
            .map_err(|e| AppError::JsonError(e).into())
    }
    
    fn write_to_file(&self, results: &[ScanResult], output_path: &Path) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let json = self.format_results(results)?;
        fs::write(output_path, json)
            .map_err(|e| AppError::IoError(e).into())
    }
}