use std::error::Error as StdError;
use std::path::Path;

/// テキスト抽出器のインターフェース
pub trait TextExtractor: Send + Sync {
    /// この抽出器がサポートするファイルタイプを返す
    fn supports(&self, file_type: &str) -> bool;
    
    /// ファイルからテキストを抽出する
    fn extract(&self, file_path: &Path) -> Result<String, Box<dyn StdError + Send + Sync>>;
}