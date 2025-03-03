use clap::{Parser, Subcommand};
use std::path::Path;
use std::error::Error as StdError;
use std::sync::Arc;

use crate::domain::{ScanService, DetectionService, OutputService};
use crate::application::{ScanUseCase, FormatUseCase};
use crate::utils::AppConfig;

#[derive(Parser)]
#[command(name = "personal-info-scanner")]
#[command(version = "0.1.0")]
#[command(about = "ファイル内の個人情報を検出するスキャナーです")]
#[command(long_about = "指定されたディレクトリ内のファイル（テキストファイルおよびPDFファイル）をスキャンし、個人情報（氏名、メールアドレス、電話番号、住所、クレジットカード番号など）が含まれているかどうかを検出します。検出には、Ollama API（デフォルトではDeepseek Coder）を使用し、結果はJSON形式で出力します。")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    // コマンドの種類を取得
    #[allow(dead_code)]
    pub fn get_command(&self) -> &Commands {
        &self.command
    }
    
    // API URLを取得
    pub fn get_api_url(&self) -> String {
        match &self.command {
            Commands::Scan { api_url, .. } => api_url.clone(),
            Commands::ScanFile { api_url, .. } => api_url.clone(),
        }
    }
    
    // モデル名を取得
    pub fn get_model(&self) -> String {
        match &self.command {
            Commands::Scan { model, .. } => model.clone(),
            Commands::ScanFile { model, .. } => model.clone(),
        }
    }
    
    // タイムアウト値を取得
    pub fn get_timeout(&self) -> u64 {
        match &self.command {
            Commands::Scan { timeout, .. } => *timeout,
            Commands::ScanFile { timeout, .. } => *timeout,
        }
    }
    
    // APIを使用しない設定かどうかを取得
    pub fn is_no_api(&self) -> bool {
        match &self.command {
            Commands::Scan { no_api, .. } => *no_api,
            Commands::ScanFile { no_api, .. } => *no_api,
        }
    }
    
    // 設定ファイルのパスを取得
    pub fn get_config_path(&self) -> Option<String> {
        match &self.command {
            Commands::Scan { config, .. } => config.clone(),
            Commands::ScanFile { config, .. } => config.clone(),
        }
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// ディレクトリをスキャンして個人情報を検出します
    Scan {
        /// スキャンするディレクトリのパス
        #[arg(default_value = ".")]
        path: String,

        /// 結果の出力先ファイル（指定なしの場合は標準出力）
        #[arg(short, long)]
        output: Option<String>,
        
        /// PDFファイルをスキャンするかどうか
        #[arg(short, long, default_value = "true")]
        pdf: bool,
        
        /// DOCXファイルをスキャンするかどうか
        #[arg(long, default_value = "true")]
        docx: bool,
        
        /// Ollama APIのエンドポイント
        #[arg(long, default_value = "http://localhost:11434/api/generate")]
        api_url: String,
        
        /// 使用するモデル
        #[arg(long, default_value = "deepseek-coder")]
        model: String,
        
        /// API呼び出しのタイムアウト（秒）
        #[arg(long, default_value = "60")]
        timeout: u64,
        
        /// API呼び出しをスキップし、正規表現のみで検出する
        #[arg(long)]
        no_api: bool,
        
        /// 詳細なログを出力する
        #[arg(short, long)]
        verbose: bool,
        
        /// 再帰的にディレクトリをスキャンする
        #[arg(short, long, default_value = "true")]
        recursive: bool,
        
        /// 設定ファイルのパス
        #[arg(long)]
        config: Option<String>,
    },
    
    /// 単一のファイルをスキャンして個人情報を検出します
    ScanFile {
        /// スキャンするファイルのパス
        file_path: String,
        
        /// 結果の出力先ファイル（指定なしの場合は標準出力）
        #[arg(short, long)]
        output: Option<String>,
        
        /// API呼び出しをスキップし、正規表現のみで検出する
        #[arg(long)]
        no_api: bool,
        
        /// 詳細なログを出力する
        #[arg(short, long)]
        verbose: bool,
        
        /// Ollama APIのエンドポイント
        #[arg(long, default_value = "http://localhost:11434/api/generate")]
        api_url: String,
        
        /// 使用するモデル
        #[arg(long, default_value = "deepseek-coder")]
        model: String,
        
        /// API呼び出しのタイムアウト（秒）
        #[arg(long, default_value = "60")]
        timeout: u64,
        
        /// 設定ファイルのパス
        #[arg(long)]
        config: Option<String>,
    },
}

/// CLIの実行
pub struct CliRunner {
    scan_use_case: ScanUseCase,
    format_use_case: FormatUseCase,
}

impl CliRunner {
    pub fn new(
        scan_service: Arc<dyn ScanService + Send + Sync>,
        detection_service: Arc<dyn DetectionService + Send + Sync>,
        output_service: Box<dyn OutputService + Send + Sync>,
        config: &AppConfig,
    ) -> Self {
        CliRunner {
            scan_use_case: ScanUseCase::new(
                scan_service, 
                detection_service,
                Some(config.max_concurrency),
            ),
            format_use_case: FormatUseCase::new(output_service),
        }
    }
    
    pub async fn run(&self, cli: Cli) -> Result<(), Box<dyn StdError + Send + Sync>> {
        match &cli.command {
            Commands::Scan { 
                path, 
                output, 
                recursive,
                verbose,
                ..
            } => {
                self.run_scan(path, output.as_deref(), *recursive, *verbose).await
            },
            Commands::ScanFile { 
                file_path, 
                output, 
                verbose,
                ..
            } => {
                self.run_scan_file(file_path, output.as_deref(), *verbose).await
            },
        }
    }
    
    async fn run_scan(&self, path: &str, output: Option<&str>, recursive: bool, verbose: bool) -> Result<(), Box<dyn StdError + Send + Sync>> {
        if verbose {
            println!("スキャン開始: {}", path);
        }
        
        // ディレクトリの存在確認
        let path_obj = Path::new(path);
        if !path_obj.exists() {
            return Err(format!("ディレクトリが存在しません: {}", path).into());
        }
        
        // スキャン実行
        let results = self.scan_use_case.scan_directory(path, recursive).await?;
        
        if verbose {
            println!("スキャン完了: {}個のファイルをスキャン, {}個の個人情報を検出", 
                results.len(),
                results.iter().map(|r| r.personal_information.len()).sum::<usize>());
        }
        
        // 結果の出力
        if let Some(output_path) = output {
            self.format_use_case.write_to_file(&results, output_path)?;
            if verbose {
                println!("結果を保存しました: {}", output_path);
            }
        } else {
            let json = self.format_use_case.format_results(&results)?;
            println!("{}", json);
        }
        
        Ok(())
    }
    
    async fn run_scan_file(&self, file_path: &str, output: Option<&str>, verbose: bool) -> Result<(), Box<dyn StdError + Send + Sync>> {
        if verbose {
            println!("ファイルをスキャン: {}", file_path);
        }
        
        // ファイルの存在確認
        let path_obj = Path::new(file_path);
        if !path_obj.exists() {
            return Err(format!("ファイルが存在しません: {}", file_path).into());
        }
        if !path_obj.is_file() {
            return Err(format!("指定されたパスはファイルではありません: {}", file_path).into());
        }
        
        // ファイルスキャン
        let scan_result = self.scan_use_case.scan_file(file_path).await?;
        
        if verbose {
            println!("スキャン完了: {}個の個人情報を検出", scan_result.personal_information.len());
        }
        
        // 結果の出力
        let results = vec![scan_result];
        if let Some(output_path) = output {
            self.format_use_case.write_to_file(&results, output_path)?;
            if verbose {
                println!("結果を保存しました: {}", output_path);
            }
        } else {
            let json = self.format_use_case.format_results(&results)?;
            println!("{}", json);
        }
        
        Ok(())
    }
}