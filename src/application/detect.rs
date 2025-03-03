use crate::domain::{DetectionService, PersonalInformationDetector, FileInfo, PersonalInformation, ScanResult};
use std::error::Error as StdError;
use std::sync::Arc;
use async_trait::async_trait;

/// 検出サービスの実装
pub struct DetectionServiceImpl {
    detector: Arc<dyn PersonalInformationDetector + Send + Sync>,
}

impl DetectionServiceImpl {
    pub fn new(detector: Arc<dyn PersonalInformationDetector + Send + Sync>) -> Self {
        DetectionServiceImpl { detector }
    }
}

#[async_trait]
impl DetectionService for DetectionServiceImpl {
    async fn detect_personal_information(&self, text: &str) -> Result<Vec<PersonalInformation>, Box<dyn StdError + Send + Sync>> {
        self.detector.detect(text).await
    }
    
    async fn detect_in_file(&self, file_info: &FileInfo) -> Result<ScanResult, Box<dyn StdError + Send + Sync>> {
        let personal_information = self.detect_personal_information(&file_info.content).await?;
        
        Ok(ScanResult {
            file: file_info.path.clone(),
            personal_information,
        })
    }
}