use crate::domain::TextExtractor;
use crate::infrastructure::error::AppError;
use std::error::Error as StdError;
use std::path::Path;

pub struct ExtractorManager {
    extractors: Vec<Box<dyn TextExtractor>>,
}

impl ExtractorManager {
    pub fn new() -> Self {
        ExtractorManager {
            extractors: Vec::new(),
        }
    }
    
    pub fn register<E: TextExtractor + 'static>(&mut self, extractor: E) {
        self.extractors.push(Box::new(extractor));
    }
    
    pub fn extract(&self, file_path: &Path) -> Result<String, Box<dyn StdError + Send + Sync>> {
        // ファイル拡張子を取得
        let extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();
            
        // 拡張子に対応する抽出器を探す
        for extractor in &self.extractors {
            if extractor.supports(&extension) {
                return extractor.extract(file_path);
            }
        }
        
        // 対応する抽出器が見つからない場合はエラー
        Err(AppError::Other(format!("No extractor found for file type: {}", extension).into()).into())
    }
    
    #[allow(dead_code)]
    pub fn list_supported_extensions(&self) -> Vec<String> {
        // テストのためにサポートしている拡張子のリストを返す
        let mut extensions = Vec::new();
        for ext in ["txt", "md", "csv", "pdf", "docx"] {
            for extractor in &self.extractors {
                if extractor.supports(ext) {
                    extensions.push(ext.to_string());
                    break;
                }
            }
        }
        extensions
    }
}