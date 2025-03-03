use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PersonalInformation {
    pub type_: String,
    pub value: String,
    pub line: u32,
    pub start: u32,
    pub end: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScanResult {
    pub file: String,
    pub personal_information: Vec<PersonalInformation>,
}

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: String,
    pub content: String,
}