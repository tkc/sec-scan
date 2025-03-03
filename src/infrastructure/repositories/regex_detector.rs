use crate::domain::PersonalInformationDetector;
use crate::domain::models::PersonalInformation;
use async_trait::async_trait;
use std::error::Error as StdError;
use regex::Regex;

pub struct RegexDetector;

impl RegexDetector {
    pub fn new() -> Self {
        RegexDetector
    }
    
    // メールアドレス検出
    fn detect_email(&self, line: &str, line_idx: u32, results: &mut Vec<PersonalInformation>) {
        let re = Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap();
        
        for mat in re.find_iter(line) {
            results.push(PersonalInformation {
                type_: "email".to_string(),
                value: mat.as_str().to_string(),
                line: line_idx + 1,
                start: mat.start() as u32,
                end: mat.end() as u32,
            });
        }
    }
    
    // 電話番号検出
    fn detect_phone_number(&self, line: &str, line_idx: u32, results: &mut Vec<PersonalInformation>) {
        // 日本の電話番号パターン (例: 090-1234-5678, 0312345678 など)
        let re = Regex::new(r"(0\d{1,4}-\d{1,4}-\d{4}|0\d{9,10})").unwrap();
        
        for mat in re.find_iter(line) {
            results.push(PersonalInformation {
                type_: "phone_number".to_string(),
                value: mat.as_str().to_string(),
                line: line_idx + 1,
                start: mat.start() as u32,
                end: mat.end() as u32,
            });
        }
    }
    
    // クレジットカード番号検出
    fn detect_credit_card(&self, line: &str, line_idx: u32, results: &mut Vec<PersonalInformation>) {
        // クレジットカード番号パターン (16桁の数字, 空白やハイフン区切りも許容)
        let re = Regex::new(r"(?:\d[ -]?){13,16}").unwrap();
        
        for mat in re.find_iter(line) {
            let card_num = mat.as_str().replace([' ', '-'], "");
            
            // Luhnアルゴリズムによる妥当性チェック
            if self.is_valid_credit_card(&card_num) {
                results.push(PersonalInformation {
                    type_: "credit_card".to_string(),
                    value: mat.as_str().to_string(),
                    line: line_idx + 1,
                    start: mat.start() as u32,
                    end: mat.end() as u32,
                });
            }
        }
    }
    
    // Luhnアルゴリズムによるクレジットカード番号の妥当性チェック
    fn is_valid_credit_card(&self, card_num: &str) -> bool {
        let digits: Vec<u32> = card_num
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect();
        
        if digits.len() < 13 || digits.len() > 19 {
            return false;
        }
        
        let mut sum = 0;
        let mut double = false;
        
        for &digit in digits.iter().rev() {
            let mut value = digit;
            if double {
                value *= 2;
                if value > 9 {
                    value -= 9;
                }
            }
            sum += value;
            double = !double;
        }
        
        sum % 10 == 0
    }
}

#[async_trait]
impl PersonalInformationDetector for RegexDetector {
    async fn detect(&self, text: &str) -> Result<Vec<PersonalInformation>, Box<dyn StdError + Send + Sync>> {
        let mut personal_info = Vec::new();
        
        // 各行ごとに処理
        for (line_idx, line) in text.lines().enumerate() {
            // メールアドレス検出
            self.detect_email(line, line_idx as u32, &mut personal_info);
            
            // 電話番号検出
            self.detect_phone_number(line, line_idx as u32, &mut personal_info);
            
            // クレジットカード番号検出
            self.detect_credit_card(line, line_idx as u32, &mut personal_info);
        }
        
        Ok(personal_info)
    }
    
    fn name(&self) -> &str {
        "Regex Detector"
    }
    
    fn is_available(&self) -> bool {
        true // 常に利用可能
    }
}