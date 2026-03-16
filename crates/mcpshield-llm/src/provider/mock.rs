use super::{LlmProvider, ProviderError};
use crate::{AnalysisResult, Classification, RecommendedAction};

/// Mock provider for testing — always returns a configurable result
pub struct MockProvider {
    pub result: AnalysisResult,
    pub available: bool,
}

impl MockProvider {
    pub fn clean() -> Self {
        Self {
            result: AnalysisResult {
                classification: Classification::Clean,
                confidence: 0.95,
                threat_type: "none".into(),
                explanation: "Mock: clean tool".into(),
                data_at_risk: vec![],
                recommended_action: RecommendedAction::Allow,
                provider_used: "mock".into(),
                cached: false,
            },
            available: true,
        }
    }

    pub fn malicious() -> Self {
        Self {
            result: AnalysisResult {
                classification: Classification::Malicious,
                confidence: 0.92,
                threat_type: "tool_poisoning".into(),
                explanation: "Mock: malicious tool detected".into(),
                data_at_risk: vec!["ssh_keys".into(), "api_tokens".into()],
                recommended_action: RecommendedAction::Block,
                provider_used: "mock".into(),
                cached: false,
            },
            available: true,
        }
    }
}

#[async_trait::async_trait]
impl LlmProvider for MockProvider {
    fn name(&self) -> &str {
        "mock"
    }

    async fn is_available(&self) -> bool {
        self.available
    }

    async fn analyze(
        &self,
        _system_prompt: &str,
        _user_prompt: &str,
    ) -> Result<AnalysisResult, ProviderError> {
        if !self.available {
            return Err(ProviderError::NotAvailable("Mock unavailable".into()));
        }
        Ok(self.result.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_clean() {
        let provider = MockProvider::clean();
        assert!(provider.is_available().await);
        let result = provider.analyze("", "").await.unwrap();
        assert_eq!(result.classification, Classification::Clean);
    }

    #[tokio::test]
    async fn test_mock_malicious() {
        let provider = MockProvider::malicious();
        let result = provider.analyze("", "").await.unwrap();
        assert_eq!(result.classification, Classification::Malicious);
        assert_eq!(result.recommended_action, RecommendedAction::Block);
    }
}
