use crate::domain::{DetectionService, ScanResult, ScanService};
use crate::utils::progress::ProgressBar;
use futures::future;
use std::error::Error as StdError;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tokio::sync::Semaphore; // ProgressBarのuse宣言を追加

/// スキャンユースケースを実装するクラス
pub struct ScanUseCase {
    scan_service: Arc<dyn ScanService + Send + Sync>,
    detection_service: Arc<dyn DetectionService + Send + Sync>,
    max_concurrency: usize,
}

impl ScanUseCase {
    pub fn new(
        scan_service: Arc<dyn ScanService + Send + Sync>,
        detection_service: Arc<dyn DetectionService + Send + Sync>,
        max_concurrency: Option<usize>,
    ) -> Self {
        ScanUseCase {
            scan_service,
            detection_service,
            max_concurrency: max_concurrency.unwrap_or(4), // デフォルトの並行処理数
        }
    }

    /// ディレクトリ内のファイルをスキャンし、個人情報を検出する
    pub async fn scan_directory(
        &self,
        path: &str,
        recursive: bool,
    ) -> Result<Vec<ScanResult>, Box<dyn StdError + Send + Sync>> {
        // パスをPathオブジェクトに変換
        let path = Path::new(path);

        // ファイルリストを取得
        let file_paths = self.scan_service.scan_path(path, recursive)?;
        println!("スキャンされたファイル数: {}", file_paths.len());

        // 並行処理のための準備
        let semaphore = Arc::new(Semaphore::new(self.max_concurrency));
        let results = Arc::new(Mutex::new(Vec::<ScanResult>::new()));

        let progress_bar = ProgressBar::new(file_paths.len() as u64); // 進捗バーを初期化
        let progress_bar_arc = Arc::new(progress_bar); // Arc でラップ

        // 各ファイルを並行処理
        let tasks = file_paths.iter().map(|file_path| {
            let scan_service = Arc::clone(&self.scan_service);
            let detection_service = Arc::clone(&self.detection_service);
            let results = Arc::clone(&results);
            let semaphore = Arc::clone(&semaphore);
            let file_path = file_path.clone();
            let progress_bar_clone_arc = Arc::clone(&progress_bar_arc); // Arc をクローン

            async move {
                // セマフォを取得（同時実行数を制限）
                let _permit = semaphore.acquire().await.unwrap();

                // ファイル処理
                match scan_service.process_file(&file_path) {
                    Ok(file_info) => {
                        match detection_service.detect_in_file(&file_info).await {
                            Ok(scan_result) => {
                                // 結果を追加
                                let mut results = results.lock().unwrap();
                                results.push(scan_result);
                            }
                            Err(e) => {
                                eprintln!(
                                    "Error detecting personal information in file {}: {}",
                                    file_path.display(),
                                    e
                                );
                            }
                        }
                        progress_bar_clone_arc.update(); // ファイルを処理するたびに進捗バーを更新
                    }
                    Err(e) => {
                        progress_bar_clone_arc.update(); // ファイルを処理するたびに進捗バーを更新
                        eprintln!("Error processing file {}: {}", file_path.display(), e);
                    }
                }
            }
        });

        // すべてのタスクを実行して完了を待つ
        future::join_all(tasks).await;
        progress_bar_arc.finish(); // スキャン完了

        // 結果を取得
        let results = Arc::try_unwrap(results)
            .map_err(|_| "Failed to unwrap Arc".to_string())?
            .into_inner()
            .map_err(|_| "Failed to get inner value from Mutex".to_string())?;

        Ok(results)
    }

    /// 単一ファイルをスキャンし、個人情報を検出する
    pub async fn scan_file(
        &self,
        path: &str,
    ) -> Result<ScanResult, Box<dyn StdError + Send + Sync>> {
        // パスをPathオブジェクトに変換
        let path = Path::new(path);

        // ファイル処理
        let file_info = self.scan_service.process_file(path)?;

        // 個人情報検出
        self.detection_service.detect_in_file(&file_info).await
    }
}
