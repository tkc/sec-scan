use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
use std::time::{Instant, Duration};
use std::fmt;

/// スキャンメトリクス
pub struct ScanMetrics {
    pub start_time: Instant,
    pub file_count: AtomicUsize,
    pub error_count: AtomicUsize,
    pub detection_count: AtomicUsize,
    pub processing_time_ms: AtomicU64,
}

impl ScanMetrics {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ScanMetrics {
            start_time: Instant::now(),
            file_count: AtomicUsize::new(0),
            error_count: AtomicUsize::new(0),
            detection_count: AtomicUsize::new(0),
            processing_time_ms: AtomicU64::new(0),
        }
    }
    
    #[allow(dead_code)]
    pub fn increment_file_count(&self) {
        self.file_count.fetch_add(1, Ordering::Relaxed);
    }
    
    #[allow(dead_code)]
    pub fn increment_error_count(&self) {
        self.error_count.fetch_add(1, Ordering::Relaxed);
    }
    
    #[allow(dead_code)]
    pub fn add_detection_count(&self, count: usize) {
        self.detection_count.fetch_add(count, Ordering::Relaxed);
    }
    
    #[allow(dead_code)]
    pub fn add_processing_time(&self, duration: Duration) {
        self.processing_time_ms.fetch_add(duration.as_millis() as u64, Ordering::Relaxed);
    }
    
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }
    
    pub fn total_files(&self) -> usize {
        self.file_count.load(Ordering::Relaxed)
    }
    
    pub fn total_errors(&self) -> usize {
        self.error_count.load(Ordering::Relaxed)
    }
    
    pub fn total_detections(&self) -> usize {
        self.detection_count.load(Ordering::Relaxed)
    }
    
    pub fn total_processing_time(&self) -> Duration {
        Duration::from_millis(self.processing_time_ms.load(Ordering::Relaxed))
    }
    
    pub fn average_time_per_file(&self) -> Option<Duration> {
        let files = self.total_files();
        if files == 0 {
            None
        } else {
            Some(Duration::from_millis(self.processing_time_ms.load(Ordering::Relaxed) / files as u64))
        }
    }
}

impl fmt::Display for ScanMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Scan Metrics:")?;
        writeln!(f, "  Total time: {:.2?}", self.elapsed())?;
        writeln!(f, "  Files processed: {}", self.total_files())?;
        writeln!(f, "  Errors: {}", self.total_errors())?;
        writeln!(f, "  Detections: {}", self.total_detections())?;
        writeln!(f, "  Processing time: {:.2?}", self.total_processing_time())?;
        
        if let Some(avg) = self.average_time_per_file() {
            writeln!(f, "  Average time per file: {:.2?}", avg)?;
        }
        
        Ok(())
    }
}