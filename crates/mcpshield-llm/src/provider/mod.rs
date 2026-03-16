pub mod anthropic;
pub mod local;
pub mod mock;
pub mod openai;

use crate::AnalysisResult;

/// Trait for LLM providers
#[async_trait::async_trait]
pub trait LlmProvider: Send + Sync {
    /// Name of this provider (for logging)
    fn name(&self) -> &str;

    /// Check if this provider is available
    async fn is_available(&self) -> bool;

    /// Send a prompt and get a structured analysis result
    async fn analyze(&self, system_prompt: &str, user_prompt: &str)
        -> Result<AnalysisResult, ProviderError>;
}

#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("Provider not available: {0}")]
    NotAvailable(String),
    #[error("Request failed: {0}")]
    RequestFailed(String),
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
    #[error("Timeout after {0}ms")]
    Timeout(u64),
}

/// Try providers in order, return first successful result
pub async fn cascade_analyze(
    providers: &[Box<dyn LlmProvider>],
    system_prompt: &str,
    user_prompt: &str,
) -> Result<AnalysisResult, ProviderError> {
    for provider in providers {
        if !provider.is_available().await {
            tracing::debug!("Provider {} not available, skipping", provider.name());
            continue;
        }

        match provider.analyze(system_prompt, user_prompt).await {
            Ok(result) => {
                tracing::info!("Analysis completed by provider: {}", provider.name());
                return Ok(result);
            }
            Err(e) => {
                tracing::warn!("Provider {} failed: {}, trying next", provider.name(), e);
                continue;
            }
        }
    }

    Err(ProviderError::NotAvailable(
        "All LLM providers failed or unavailable".to_string(),
    ))
}
