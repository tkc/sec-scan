use std::sync::Arc;
use clap::Parser;

mod domain;
mod application;
mod infrastructure;
mod interfaces;
mod utils;

use interfaces::Cli;
use infrastructure::{
    OllamaClient, ApiDetector, RegexDetector, HybridDetector, 
    JsonOutputFormatter, FileSystemScanner, PdfExtractor, 
    DocxExtractor, PlainTextExtractor, ExtractorManager
};
use application::DetectionServiceImpl;
use utils::AppConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // CLIの解析
    let cli = Cli::parse();
    
    // 設定ファイルの読み込み
    let config = match &cli.get_config_path() {
        Some(path) => AppConfig::from_file(path)?,
        None => AppConfig::default(),
    };
    
    // 抽出器マネージャーの設定
    let mut extractor_manager = ExtractorManager::new();
    extractor_manager.register(PlainTextExtractor);
    extractor_manager.register(PdfExtractor);
    extractor_manager.register(DocxExtractor);
    
    // ファイルシステムスキャナーの作成
    let scan_service = Arc::new(FileSystemScanner::new(extractor_manager));
    
    // 検出器の選択と作成
    let detector: Arc<dyn domain::PersonalInformationDetector + Send + Sync> = if cli.is_no_api() {
        // 正規表現のみの検出
        Arc::new(RegexDetector::new())
    } else {
        // APIクライアントの作成
        let api_client = Box::new(OllamaClient::new(
            &cli.get_api_url(),
            &cli.get_model(),
            Some(cli.get_timeout() * 1000),
        ));
        
        // ハイブリッド検出器の作成と設定
        let mut hybrid_detector = HybridDetector::new();
        
        // APIベースの検出器を追加
        hybrid_detector.add_detector(Box::new(ApiDetector::new(api_client)));
        
        // フォールバック用に正規表現検出器も追加
        hybrid_detector.add_detector(Box::new(RegexDetector::new()));
        
        Arc::new(hybrid_detector)
    };
    
    // 検出サービスの作成
    let detection_service = Arc::new(DetectionServiceImpl::new(detector));
    
    // 出力サービスの作成
    let output_service = Box::new(JsonOutputFormatter::new());
    
    // CLIランナーの作成と実行
    let runner = interfaces::CliRunner::new(
        scan_service,
        detection_service,
        output_service,
        &config,
    );
    
    // コマンドの実行
    runner.run(cli).await
}