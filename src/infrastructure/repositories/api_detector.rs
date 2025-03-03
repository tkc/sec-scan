use crate::domain::PersonalInformationDetector;
use crate::domain::models::PersonalInformation;
use crate::infrastructure::api::ApiClient;
use crate::infrastructure::error::AppError;
use async_trait::async_trait;
use std::error::Error as StdError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ApiPersonalInfo {
    #[serde(rename = "type")]
    type_: String,
    value: String,
    line: u32,
    start: u32,
    end: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    personal_information: Vec<ApiPersonalInfo>,
}

pub struct ApiDetector {
    client: Box<dyn ApiClient + Send + Sync>,
}

impl ApiDetector {
    pub fn new(client: Box<dyn ApiClient + Send + Sync>) -> Self {
        ApiDetector { client }
    }
    
    // APIレスポンスを解析し、PersonalInformation構造体のVecを返す
    fn parse_api_response(&self, response: &str, original_text: &str) -> Result<Vec<PersonalInformation>, Box<dyn StdError + Send + Sync>> {
        // まずJSONとして解析を試みる
        match serde_json::from_str::<ApiResponse>(response) {
            Ok(parsed) => {
                // 正常にJSONとして解析できた場合
                let personal_info = parsed.personal_information.into_iter()
                    .map(|info| PersonalInformation {
                        type_: info.type_,
                        value: info.value,
                        line: info.line,
                        start: info.start,
                        end: info.end,
                    })
                    .collect();
                
                Ok(personal_info)
            },
            Err(_) => {
                // JSONとして解析できなかった場合、レスポンス内のJSONを探す
                self.extract_json_from_text(response)
                    .and_then(|json_str| self.parse_api_response(json_str, original_text))
                    .or_else(|_| {
                        // JSONとして解析できない場合は、正規表現でフォールバック
                        self.detect_with_regex(original_text)
                    })
            }
        }
    }
    
    // テキスト内からJSONを抽出する
    fn extract_json_from_text<'a>(&self, text: &'a str) -> Result<&'a str, Box<dyn StdError + Send + Sync>> {
        // JSON開始位置（{）と終了位置（}）を探す
        if let (Some(start), Some(end)) = (text.find('{'), text.rfind('}')) {
            if start < end {
                return Ok(&text[start..=end]);
            }
        }
        
        Err(AppError::ApiError("No valid JSON found in response".to_string()).into())
    }
    
    // 正規表現を使用して個人情報を検出するフォールバック実装
    fn detect_with_regex(&self, text: &str) -> Result<Vec<PersonalInformation>, Box<dyn StdError + Send + Sync>> {
        // ここでは簡単な実装として正規表現ベースの検出にフォールバック
        // 実際の実装ではRegexDetectorを呼び出すか、共通のロジックを使用する
        
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
    
    // メールアドレス検出
    fn detect_email(&self, line: &str, line_idx: u32, results: &mut Vec<PersonalInformation>) {
        let re = regex::Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap();
        
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
        let re = regex::Regex::new(r"(0\d{1,4}-\d{1,4}-\d{4}|0\d{9,10})").unwrap();
        
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
        let re = regex::Regex::new(r"(?:\d[ -]?){13,16}").unwrap();
        
        for mat in re.find_iter(line) {
            let card_num = mat.as_str().replace([' ', '-'], "");
            
            // Luhnアルゴリズムによる妥当性チェック (簡易版)
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
impl PersonalInformationDetector for ApiDetector {
    async fn detect(&self, text: &str) -> Result<Vec<PersonalInformation>, Box<dyn StdError + Send + Sync>> {
        // APIを呼び出し
        let api_response = self.client.call(text).await?;
        
        // APIレスポンスの解析
        self.parse_api_response(&api_response, text)
    }
    
    fn name(&self) -> &str {
        "API Detector"
    }
    
    fn is_available(&self) -> bool {
        true // 常に利用可能だが、実際にはAPI呼び出し時にエラーが発生する可能性がある
    }
}