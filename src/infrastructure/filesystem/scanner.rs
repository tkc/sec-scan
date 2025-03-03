use crate::domain::models::FileInfo;
use crate::domain::service::ScanService;
use crate::infrastructure::error::AppError;
use crate::infrastructure::extractors::ExtractorManager;
use crate::utils::progress::ProgressBar; // ProgressBarのuse宣言を追加
use std::error::Error as StdError;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct FileSystemScanner {
    extractor_manager: ExtractorManager,
}

impl FileSystemScanner {
    pub fn new(extractor_manager: ExtractorManager) -> Self {
        FileSystemScanner { extractor_manager }
    }
    
    // ファイル拡張子が対応しているかどうかを確認
    fn is_supported_extension(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            // 対応する拡張子リスト
            let supported_extensions = ["txt", "md", "csv", "pdf", "docx"];
            return supported_extensions.contains(&ext.to_lowercase().as_str());
        }
        false
    }
}

impl ScanService for FileSystemScanner {
    fn scan_path(&self, path: &Path, recursive: bool) -> Result<Vec<PathBuf>, Box<dyn StdError + Send + Sync>> {
        // パスが存在するか確認
        if !path.exists() {
            return Err(AppError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Path not found: {}", path.display()),
            )).into());
        }
        
        let mut file_paths = Vec::new();
        let mut files_to_scan = Vec::new(); // スキャン対象のファイルを一時的に保存するVec

        if path.is_file() {
            // 単一ファイルの場合
            if self.is_supported_extension(path) {
                files_to_scan.push(path.to_path_buf());
            }
        } else if path.is_dir() {
            // ディレクトリの場合
            let walker = if recursive {
                WalkDir::new(path).follow_links(true).into_iter()
            } else {
                // 再帰的でない場合は深さ1に制限
                WalkDir::new(path).max_depth(1).follow_links(true).into_iter()
            };
            
            for entry in walker.filter_map(|e| e.ok()) {
                let entry_path = entry.path();
                if entry_path.is_file() && self.is_supported_extension(entry_path) {
                    files_to_scan.push(entry_path.to_path_buf());
                }
            }
        }

        // 進捗バーの初期化
        let total_files = files_to_scan.len() as u64;
        let progress_bar = ProgressBar::new(total_files);
        
        for file_path in files_to_scan {
            file_paths.push(file_path);
            progress_bar.update(); // ファイルを処理するたびに進捗バーを更新
        }
        progress_bar.finish(); // スキャン完了

        Ok(file_paths)
    }
    
    fn process_file(&self, file_path: &Path) -> Result<FileInfo, Box<dyn StdError + Send + Sync>> {
        // ファイルパスが存在するか確認
        if !file_path.exists() {
            return Err(AppError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found: {}", file_path.display()),
            )).into());
        }
        
        // 進捗バーの初期化
        let total_files = 1; // process_fileは単一ファイルを処理するので、total_filesは1
        let progress_bar = ProgressBar::new(total_files);

        
        // ファイル拡張子を取得して抽出マネージャーでテキスト抽出
        let content = self.extractor_manager.extract(file_path)?;
        
        // 進捗バーを更新
        progress_bar.update();
        progress_bar.finish();

        Ok(FileInfo {
            path: file_path.to_string_lossy().to_string(),
            content,
        })
    }
}
