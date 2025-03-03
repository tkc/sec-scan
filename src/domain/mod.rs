pub mod models;
pub mod detector;
pub mod extractor;
pub mod service;

// Re-export commonly used types
pub use models::{FileInfo, PersonalInformation, ScanResult};
pub use detector::PersonalInformationDetector;
pub use extractor::TextExtractor;
pub use service::{ScanService, DetectionService, OutputService};