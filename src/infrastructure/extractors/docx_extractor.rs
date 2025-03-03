use crate::domain::TextExtractor;
use crate::infrastructure::error::AppError;
use std::error::Error as StdError;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use zip::ZipArchive;
use regex::Regex;

pub struct DocxExtractor;

impl TextExtractor for DocxExtractor {
    fn supports(&self, file_type: &str) -> bool {
        file_type.to_lowercase() == "docx"
    }
    
    fn extract(&self, file_path: &Path) -> Result<String, Box<dyn StdError + Send + Sync>> {
        // ファイルを開く
        let file = File::open(file_path)
            .map_err(|e| AppError::IoError(e))?;
        
        // ZIPアーカイブとして開く
        let mut archive = ZipArchive::new(file)
            .map_err(|e| AppError::DocxExtractError(format!("Failed to open DOCX as ZIP: {}", e)))?;
        
        // document.xmlを取得（DOCXの本文）
        let mut document_content = String::new();
        if let Ok(mut document) = archive.by_name("word/document.xml") {
            document.read_to_string(&mut document_content)
                .map_err(|e| AppError::DocxExtractError(format!("Failed to read document.xml: {}", e)))?;
        } else {
            return Err(AppError::DocxExtractError("document.xml not found in DOCX".to_string()).into());
        }
        
        // XMLからテキストを抽出（簡易実装）
        // 正規表現で<w:t>...</w:t>タグ内のテキストを抽出
        let re = Regex::new(r"<w:t[^>]*>(.*?)</w:t>").unwrap();
        let mut text = String::new();
        
        for cap in re.captures_iter(&document_content) {
            if let Some(matched) = cap.get(1) {
                text.push_str(matched.as_str());
                text.push(' '); // 単語間のスペースを追加
            }
        }
        
        // 段落の区切りを追加
        let paragraph_re = Regex::new(r"</w:p>").unwrap();
        let text = paragraph_re.replace_all(&text, "\n");
        
        Ok(text.to_string())
    }
}