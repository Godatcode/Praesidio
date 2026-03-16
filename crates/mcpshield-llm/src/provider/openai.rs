use reqwest::Client;
use serde::Serialize;
use std::time::Duration;

use super::local::parse_analysis_response;
use super::{LlmProvider, ProviderError};
use crate::{AnalysisResult, ApiConfig};

/// OpenAI API provider
pub struct OpenAIProvider {
    client: Client,
    api_key: Option<String>,
    model: String,
    max_tokens: u32,
}

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<OpenAIMessage>,
    response_format: ResponseFormat,
}

#[derive(Serialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ResponseFormat {
    #[serde(rename = "type")]
    format_type: String,
}

impl OpenAIProvider {
    pub fn new(config: &ApiConfig) -> Self {
        let api_key = std::env::var("OPENAI_API_KEY").ok();

        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .unwrap_or_default(),
            api_key,
            model: if config.model.is_empty() {
                "gpt-4o-mini".to_string()
            } else {
                config.model.clone()
            },
            max_tokens: config.max_tokens,
        }
    }
}

#[async_trait::async_trait]
impl LlmProvider for OpenAIProvider {
    fn name(&self) -> &str {
        "openai"
    }

    async fn is_available(&self) -> bool {
        self.api_key.is_some()
    }

    async fn analyze(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<AnalysisResult, ProviderError> {
        let api_key = self
            .api_key
            .as_ref()
            .ok_or_else(|| ProviderError::NotAvailable("OPENAI_API_KEY not set".into()))?;

        let request = OpenAIRequest {
            model: self.model.clone(),
            max_tokens: self.max_tokens,
            messages: vec![
                OpenAIMessage {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                OpenAIMessage {
                    role: "user".to_string(),
                    content: user_prompt.to_string(),
                },
            ],
            response_format: ResponseFormat {
                format_type: "json_object".to_string(),
            },
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| ProviderError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(ProviderError::RequestFailed(format!(
                "HTTP {}: {}",
                status, body
            )));
        }

        let body: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ProviderError::InvalidResponse(e.to_string()))?;

        let text = body
            .get("choices")
            .and_then(|c| c.as_array())
            .and_then(|arr| arr.first())
            .and_then(|choice| choice.get("message"))
            .and_then(|msg| msg.get("content"))
            .and_then(|t| t.as_str())
            .ok_or_else(|| ProviderError::InvalidResponse("No content in response".into()))?;

        parse_analysis_response(text, "openai")
    }
}
