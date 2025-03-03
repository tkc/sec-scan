pub mod pdf_extractor;
pub mod docx_extractor;
pub mod text_extractor;
pub mod extractor_manager;

pub use pdf_extractor::PdfExtractor;
pub use docx_extractor::DocxExtractor;
pub use text_extractor::PlainTextExtractor;
pub use extractor_manager::ExtractorManager;