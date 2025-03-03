use crate::domain::TextExtractor;
use crate::infrastructure::error::AppError;
use std::error::Error as StdError;
use std::path::Path;

pub struct PdfExtractor;

impl TextExtractor for PdfExtractor {
    fn supports(&self, file_type: &str) -> bool {
        file_type.to_lowercase() == "pdf"
    }
    
    fn extract(&self, file_path: &Path) -> Result<String, Box<dyn StdError + Send + Sync>> {
        // ファイルパスのチェック（使用されないが、無効な場合はエラーを返す）
        file_path.to_str()
            .ok_or_else(|| AppError::Other("Invalid file path".into()))?;
            
        // PDFからテキストを抽出
        pdf_extract::extract_text(file_path)
            .map_err(|e| AppError::PdfExtractError(e.to_string()).into())
    }
}