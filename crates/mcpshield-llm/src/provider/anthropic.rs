use reqwest::Client;
use serde::Serialize;
use std::time::Duration;

use super::local::parse_analysis_response;
use super::{LlmProvider, ProviderError};
use crate::{AnalysisResult, ApiConfig};

/// Anthropic Claude API provider
pub struct AnthropicProvider {
    client: Client,
    api_key: Option<String>,
    model: String,
    max_tokens: u32,
}

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    system: String,
    messages: Vec<AnthropicMessage>,
}

#[derive(Serialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

impl AnthropicProvider {
    pub fn new(config: &ApiConfig) -> Self {
        let api_key = std::env::var("ANTHROPIC_API_KEY").ok();

        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .unwrap_or_default(),
            api_key,
            model: if config.model.is_empty() {
                "claude-sonnet-4-20250514".to_string()
            } else {
                config.model.clone()
            },
            max_tokens: config.max_tokens,
        }
    }
}

#[async_trait::async_trait]
impl LlmProvider for AnthropicProvider {
    fn name(&self) -> &str {
        "anthropic-claude"
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
            .ok_or_else(|| ProviderError::NotAvailable("ANTHROPIC_API_KEY not set".into()))?;

        let request = AnthropicRequest {
            model: self.model.clone(),
            max_tokens: self.max_tokens,
            system: system_prompt.to_string(),
            messages: vec![AnthropicMessage {
                role: "user".to_string(),
                content: user_prompt.to_string(),
            }],
        };

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
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
            .get("content")
            .and_then(|c| c.as_array())
            .and_then(|arr| arr.first())
            .and_then(|block| block.get("text"))
            .and_then(|t| t.as_str())
            .ok_or_else(|| ProviderError::InvalidResponse("No text in response".into()))?;

        parse_analysis_response(text, "anthropic-claude")
    }
}
