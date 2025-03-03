pub mod error;
pub mod api;
pub mod extractors;
pub mod filesystem;
pub mod repositories;

#[allow(unused_imports)]
pub use error::AppError;

pub use api::OllamaClient;
pub use extractors::{
    PdfExtractor, 
    DocxExtractor, 
    PlainTextExtractor, 
    ExtractorManager
};
pub use filesystem::FileSystemScanner;
pub use repositories::{
    ApiDetector,
    RegexDetector,
    HybridDetector,
    JsonOutputFormatter
};