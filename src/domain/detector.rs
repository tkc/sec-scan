use crate::domain::models::PersonalInformation;
use async_trait::async_trait;
use std::error::Error as StdError;

/// 個人情報検出器のインターフェース
#[async_trait]
pub trait PersonalInformationDetector {
    /// テキスト内の個人情報を検出する
    async fn detect(&self, text: &str) -> Result<Vec<PersonalInformation>, Box<dyn StdError + Send + Sync>>;
    
    /// 検出器の名前を返す
    fn name(&self) -> &str;
    
    /// この検出器が使用可能かどうかを返す
    fn is_available(&self) -> bool {
        true
    }
}