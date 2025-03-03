use crate::domain::models::{FileInfo, PersonalInformation, ScanResult};
use std::error::Error as StdError;
use std::path::{Path, PathBuf};
use async_trait::async_trait;

/// スキャンサービスのトレイト
pub trait ScanService {
    /// 指定されたパスをスキャンし、ファイル情報のリストを返す
    fn scan_path(&self, path: &Path, recursive: bool) -> Result<Vec<PathBuf>, Box<dyn StdError + Send + Sync>>;
    
    /// ファイルを処理して内容を抽出する
    fn process_file(&self, file_path: &Path) -> Result<FileInfo, Box<dyn StdError + Send + Sync>>;
}

/// 個人情報検出サービスのトレイト
#[async_trait]
pub trait DetectionService {
    /// テキスト内の個人情報を検出する
    async fn detect_personal_information(&self, text: &str) -> Result<Vec<PersonalInformation>, Box<dyn StdError + Send + Sync>>;
    
    /// ファイルの内容から個人情報を検出する
    async fn detect_in_file(&self, file_info: &FileInfo) -> Result<ScanResult, Box<dyn StdError + Send + Sync>>;
}

/// 出力フォーマットサービスのトレイト
pub trait OutputService {
    /// 結果をフォーマットして文字列として返す
    fn format_results(&self, results: &[ScanResult]) -> Result<String, Box<dyn StdError + Send + Sync>>;
    
    /// 結果をファイルに出力する
    fn write_to_file(&self, results: &[ScanResult], output_path: &Path) -> Result<(), Box<dyn StdError + Send + Sync>>;
}