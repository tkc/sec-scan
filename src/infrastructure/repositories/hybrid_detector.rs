use crate::domain::PersonalInformationDetector;
use crate::domain::models::PersonalInformation;
use async_trait::async_trait;
use std::error::Error as StdError;
use std::collections::HashSet;

pub struct HybridDetector {
    detectors: Vec<Box<dyn PersonalInformationDetector + Send + Sync>>,
}

impl HybridDetector {
    pub fn new() -> Self {
        HybridDetector {
            detectors: Vec::new(),
        }
    }
    
    pub fn add_detector(&mut self, detector: Box<dyn PersonalInformationDetector + Send + Sync>) {
        self.detectors.push(detector);
    }
    
    // 重複を除去して結果をマージする
    fn merge_results(&self, multiple_results: Vec<Vec<PersonalInformation>>) -> Vec<PersonalInformation> {
        let mut result = Vec::new();
        let mut seen = HashSet::new();
        
        for detector_results in multiple_results {
            for info in detector_results {
                // 既に同じ情報が含まれていないか確認
                let key = format!("{}:{}:{}:{}", info.type_, info.value, info.line, info.start);
                if !seen.contains(&key) {
                    seen.insert(key);
                    result.push(info);
                }
            }
        }
        
        result
    }
}

#[async_trait]
impl PersonalInformationDetector for HybridDetector {
    async fn detect(&self, text: &str) -> Result<Vec<PersonalInformation>, Box<dyn StdError + Send + Sync>> {
        if self.detectors.is_empty() {
            return Ok(Vec::new());
        }
        
        let mut results = Vec::new();
        let mut errors = Vec::new();
        
        // すべての検出器を実行
        for detector in &self.detectors {
            if detector.is_available() {
                match detector.detect(text).await {
                    Ok(detector_result) => {
                        // 成功した場合は結果を追加
                        println!("Detector '{}' found {} personal information items", 
                            detector.name(), detector_result.len());
                        results.push(detector_result);
                    }
                    Err(e) => {
                        // エラーが発生した場合はログに記録
                        println!("Detector '{}' failed: {}", detector.name(), e);
                        errors.push(format!("{}:{}", detector.name(), e));
                    }
                }
            }
        }
        
        // 少なくとも1つの検出器が成功していれば結果を返す
        if !results.is_empty() {
            Ok(self.merge_results(results))
        } else if !errors.is_empty() {
            // すべての検出器が失敗した場合はエラーを返す
            Err(format!("All detectors failed: {}", errors.join(", ")).into())
        } else {
            // 利用可能な検出器がなかった場合は空の結果を返す
            Ok(Vec::new())
        }
    }
    
    fn name(&self) -> &str {
        "Hybrid Detector"
    }
    
    fn is_available(&self) -> bool {
        // 少なくとも1つの検出器が利用可能であればtrue
        self.detectors.iter().any(|d| d.is_available())
    }
}