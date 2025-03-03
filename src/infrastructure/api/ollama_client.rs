use crate::infrastructure::error::AppError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use std::time::{Duration, Instant};
use async_trait::async_trait;

// APIクライアントのトレイト
#[async_trait]
pub trait ApiClient {
    async fn call(&self, text: &str) -> Result<String, Box<dyn StdError + Send + Sync>>;
    #[allow(unused)]
    fn get_url(&self) -> &str;
    #[allow(unused)]
    fn get_model(&self) -> &str;
}

// Ollamaリクエスト構造体
#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    format: Option<String>,
}

// Ollamaレスポンス構造体
#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

// Ollamaストリームレスポンス構造体
#[allow(dead_code)]
#[derive(Deserialize)]
struct OllamaStreamResponse {
    response: String,
    done: bool,
}

// 定数
const DEFAULT_TIMEOUT_MS: u64 = 60000; // 60秒
const MAX_RETRIES: u32 = 3;
const RETRY_DELAY_MS: u64 = 1000;

// Ollamaクライアント実装
pub struct OllamaClient {
    api_url: String,
    model: String,
    timeout_ms: u64,
    client: Client,
}

impl OllamaClient {
    pub fn new(api_url: &str, model: &str, timeout_ms: Option<u64>) -> Self {
        OllamaClient {
            api_url: api_url.to_string(),
            model: model.to_string(),
            timeout_ms: timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS),
            client: Client::new(),
        }
    }
    
    // 個人情報検出用プロンプトを生成
    fn create_personal_info_prompt(&self, text: &str) -> String {
        format!(
            "以下のテキストを解析し、含まれている個人情報（氏名、メールアドレス、電話番号、住所、クレジットカード番号など）を検出してください。\
            検出結果はJSON形式で返してください。各項目について、種類（type）、値（value）、行番号（line）、開始位置（start）、終了位置（end）を含めてください。\
            \n\nテキスト:\n{}",
            text
        )
    }
}

#[async_trait]
impl ApiClient for OllamaClient {
    async fn call(&self, text: &str) -> Result<String, Box<dyn StdError + Send + Sync>> {
        let mut tries = 0;
        let mut last_error = None;
        
        // プロンプトを生成
        let prompt = self.create_personal_info_prompt(text);
        
        // リクエストボディを作成
        let request_body = OllamaRequest {
            model: self.model.clone(),
            prompt,
            format: Some("json".to_string()),
        };
        
        while tries < MAX_RETRIES {
            let start_time = Instant::now();
            
            // APIリクエスト
            match self.client.post(&self.api_url)
                .json(&request_body)
                .timeout(Duration::from_millis(self.timeout_ms))
                .send()
                .await {
                    Ok(response) => {
                        if response.status().is_success() {
                            // レスポンスをJSONとしてパース
                            match response.json::<OllamaResponse>().await {
                                Ok(result) => {
                                    let elapsed = start_time.elapsed();
                                    println!("API request completed in {:.2?}", elapsed);
                                    return Ok(result.response);
                                }
                                Err(e) => {
                                    last_error = Some(format!("Failed to parse API response: {}", e));
                                }
                            }
                        } else {
                            last_error = Some(format!("API returned error status: {}", response.status()));
                        }
                    }
                    Err(e) => {
                        last_error = Some(format!("API request failed: {}", e));
                    }
                }
            
            tries += 1;
            if tries < MAX_RETRIES {
                println!("Retrying API call ({}/{}), waiting for {}ms...", tries, MAX_RETRIES, RETRY_DELAY_MS);
                tokio::time::sleep(Duration::from_millis(RETRY_DELAY_MS)).await;
            }
        }
        
        Err(AppError::ApiError(last_error.unwrap_or_else(|| "Unknown API error".to_string())).into())
    }
    
    fn get_url(&self) -> &str {
        &self.api_url
    }
    
    fn get_model(&self) -> &str {
        &self.model
    }
}