// コードベースは完全に新しいアーキテクチャに移行されました
// モジュールのエクスポート
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod interfaces;
pub mod utils;

// 主要コンポーネントの再エクスポート
pub use domain::{
    FileInfo, PersonalInformation, ScanResult,
    PersonalInformationDetector, TextExtractor,
    ScanService, DetectionService, OutputService
};

pub use application::{
    ScanUseCase, DetectionServiceImpl, FormatUseCase
};

pub use infrastructure::{
    OllamaClient,
    PdfExtractor, DocxExtractor, PlainTextExtractor, ExtractorManager,
    FileSystemScanner,
    ApiDetector, RegexDetector, HybridDetector,
    JsonOutputFormatter
};

pub use interfaces::{
    Cli, CliRunner
};

pub use utils::AppConfig;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_scan_and_process() {
        // テスト用ディレクトリ作成
        let dir = tempdir().unwrap();
        
        // テキストファイル作成
        let text_file_path = dir.path().join("test.txt");
        let mut text_file = File::create(&text_file_path).unwrap();
        writeln!(text_file, "This is a test file with an email: test@example.com").unwrap();
        
        // 抽出器マネージャーの設定
        let mut extractor_manager = ExtractorManager::new();
        extractor_manager.register(PlainTextExtractor);
        
        // ファイルシステムスキャナーの作成
        let scan_service = Arc::new(FileSystemScanner::new(extractor_manager));
        
        // 正規表現ベースの検出器を作成
        let detector = Arc::new(RegexDetector::new());
        
        // 検出サービスの作成
        let detection_service = Arc::new(DetectionServiceImpl::new(detector));
        
        // スキャンユースケースの作成
        let scan_use_case = ScanUseCase::new(
            scan_service,
            detection_service,
            Some(1),
        );
        
        // スキャン実行
        let results = scan_use_case.scan_directory(dir.path().to_str().unwrap(), true).await.unwrap();
        
        // 検証
        assert_eq!(results.len(), 1);
        let personal_info = &results[0].personal_information;
        assert!(!personal_info.is_empty());
        
        let email_info = personal_info.iter().find(|info| info.type_ == "email").unwrap();
        assert_eq!(email_info.value, "test@example.com");
    }
}
