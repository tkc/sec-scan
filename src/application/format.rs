use crate::domain::{OutputService, ScanResult};
use std::error::Error as StdError;
use std::path::Path;

/// 出力フォーマットユースケース
pub struct FormatUseCase {
    output_service: Box<dyn OutputService + Send + Sync>,
}

impl FormatUseCase {
    pub fn new(output_service: Box<dyn OutputService + Send + Sync>) -> Self {
        FormatUseCase { output_service }
    }
    
    /// 結果を文字列として取得
    pub fn format_results(&self, results: &[ScanResult]) -> Result<String, Box<dyn StdError + Send + Sync>> {
        self.output_service.format_results(results)
    }
    
    /// 結果をファイルに出力
    pub fn write_to_file(&self, results: &[ScanResult], output_path: &str) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let path = Path::new(output_path);
        self.output_service.write_to_file(results, path)
    }
}