use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use std::path::Path;
use std::fs;
use std::collections::HashMap;

/// アプリケーション設定
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub api_url: String,
    pub model_name: String,
    pub timeout_ms: u64,
    pub max_concurrency: usize,
    pub supported_file_types: Vec<String>,
    pub detection_patterns: HashMap<String, Vec<String>>,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            api_url: "http://localhost:11434/api/generate".to_string(),
            model_name: "deepseek-coder".to_string(),
            timeout_ms: 60000,
            max_concurrency: 4,
            supported_file_types: vec![
                "txt".to_string(),
                "md".to_string(),
                "csv".to_string(),
                "pdf".to_string(),
                "docx".to_string(),
            ],
            detection_patterns: {
                let mut patterns = HashMap::new();
                patterns.insert(
                    "email".to_string(),
                    vec![r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}".to_string()],
                );
                patterns.insert(
                    "phone_number".to_string(),
                    vec![r"(0\d{1,4}-\d{1,4}-\d{4}|0\d{9,10})".to_string()],
                );
                patterns.insert(
                    "credit_card".to_string(),
                    vec![r"(?:\d[ -]?){13,16}".to_string()],
                );
                patterns
            },
        }
    }
}

impl AppConfig {
    /// 設定ファイルから読み込む
    pub fn from_file(path: &str) -> Result<Self, Box<dyn StdError + Send + Sync>> {
        let path = Path::new(path);
        if !path.exists() {
            return Ok(AppConfig::default());
        }
        
        let content = fs::read_to_string(path)?;
        let config = serde_json::from_str(&content)?;
        
        Ok(config)
    }
    
    /// 設定ファイルに保存
    #[allow(dead_code)]
    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        
        Ok(())
    }
}