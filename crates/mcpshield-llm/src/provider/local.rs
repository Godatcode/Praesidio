use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::{LlmProvider, ProviderError};
use crate::{AnalysisResult, Classification, LocalConfig, RecommendedAction};

/// Ollama local LLM provider
pub struct OllamaProvider {
    client: Client,
    endpoint: String,
    model: String,
    timeout_ms: u64,
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    system: String,
    stream: bool,
    format: String,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

impl OllamaProvider {
    pub fn new(config: &LocalConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_millis(config.timeout_ms))
            .build()
            .unwrap_or_default();

        Self {
            client,
            endpoint: config.endpoint.clone(),
            model: config.model.clone(),
            timeout_ms: config.timeout_ms,
        }
    }
}

#[async_trait::async_trait]
impl LlmProvider for OllamaProvider {
    fn name(&self) -> &str {
        "ollama-local"
    }

    async fn is_available(&self) -> bool {
        let url = format!("{}/api/tags", self.endpoint);
        self.client.get(&url).send().await.is_ok()
    }

    async fn analyze(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<AnalysisResult, ProviderError> {
        let url = format!("{}/api/generate", self.endpoint);

        let request = OllamaRequest {
            model: self.model.clone(),
            prompt: user_prompt.to_string(),
            system: system_prompt.to_string(),
            stream: false,
            format: "json".to_string(),
        };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    ProviderError::Timeout(self.timeout_ms)
                } else {
                    ProviderError::RequestFailed(e.to_string())
                }
            })?;

        if !response.status().is_success() {
            return Err(ProviderError::RequestFailed(format!(
                "HTTP {}",
                response.status()
            )));
        }

        let ollama_resp: OllamaResponse = response
            .json()
            .await
            .map_err(|e| ProviderError::InvalidResponse(e.to_string()))?;

        parse_analysis_response(&ollama_resp.response, "ollama-local")
    }
}

/// Parse JSON response from any LLM into AnalysisResult
pub fn parse_analysis_response(
    text: &str,
    provider_name: &str,
) -> Result<AnalysisResult, ProviderError> {
    // Try to extract JSON from the response (LLMs sometimes add prose around it)
    let json_str = extract_json(text).unwrap_or(text);

    let parsed: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| ProviderError::InvalidResponse(format!("JSON parse error: {}", e)))?;

    let classification = match parsed
        .get("classification")
        .and_then(|v| v.as_str())
        .unwrap_or("SUSPICIOUS")
    {
        "CLEAN" | "clean" => Classification::Clean,
        "MALICIOUS" | "malicious" => Classification::Malicious,
        _ => Classification::Suspicious,
    };

    let confidence = parsed
        .get("confidence")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.5);

    let threat_type = parsed
        .get("threat_type")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();

    let explanation = parsed
        .get("explanation")
        .and_then(|v| v.as_str())
        .unwrap_or("No explanation provided")
        .to_string();

    let data_at_risk = parsed
        .get("data_at_risk")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let recommended_action = match parsed
        .get("recommended_action")
        .and_then(|v| v.as_str())
        .unwrap_or("warn")
    {
        "allow" => RecommendedAction::Allow,
        "block" => RecommendedAction::Block,
        _ => RecommendedAction::Warn,
    };

    Ok(AnalysisResult {
        classification,
        confidence,
        threat_type,
        explanation,
        data_at_risk,
        recommended_action,
        provider_used: provider_name.to_string(),
        cached: false,
    })
}

/// Extract JSON object from text that may contain surrounding prose
fn extract_json(text: &str) -> Option<&str> {
    let start = text.find('{')?;
    let mut depth = 0;
    let bytes = text.as_bytes();
    for i in start..bytes.len() {
        match bytes[i] {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&text[start..=i]);
                }
            }
            _ => {}
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_clean_response() {
        let json = r#"{"classification":"CLEAN","confidence":0.95,"threat_type":"none","explanation":"This is a normal tool","data_at_risk":[],"recommended_action":"allow"}"#;
        let result = parse_analysis_response(json, "test").unwrap();
        assert_eq!(result.classification, Classification::Clean);
        assert_eq!(result.recommended_action, RecommendedAction::Allow);
    }

    #[test]
    fn test_parse_malicious_response() {
        let json = r#"{"classification":"MALICIOUS","confidence":0.92,"threat_type":"tool_poisoning","explanation":"Hidden instructions to steal SSH keys","data_at_risk":["ssh_keys"],"recommended_action":"block"}"#;
        let result = parse_analysis_response(json, "test").unwrap();
        assert_eq!(result.classification, Classification::Malicious);
        assert_eq!(result.recommended_action, RecommendedAction::Block);
        assert!(result.data_at_risk.contains(&"ssh_keys".to_string()));
    }

    #[test]
    fn test_extract_json_from_prose() {
        let text = "Here is my analysis:\n{\"classification\":\"CLEAN\"}\nHope that helps!";
        let json = extract_json(text).unwrap();
        assert_eq!(json, r#"{"classification":"CLEAN"}"#);
    }
}
