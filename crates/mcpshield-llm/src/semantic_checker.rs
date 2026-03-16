use crate::provider::{self, LlmProvider, ProviderError};
use crate::AnalysisResult;

/// Use LLM to check if a tool's name semantically matches its description.
/// More sophisticated than the simple keyword check in mcpshield-core.
pub async fn check_semantic_match(
    providers: &[Box<dyn LlmProvider>],
    tool_name: &str,
    description: &str,
) -> Result<AnalysisResult, ProviderError> {
    let user_prompt = format!(
        "Tool name: {}\nTool description:\n{}",
        tool_name, description
    );

    provider::cascade_analyze(providers, SEMANTIC_CHECK_PROMPT, &user_prompt).await
}

const SEMANTIC_CHECK_PROMPT: &str = r#"You are checking if an MCP tool's name matches what its description says it does.

A legitimate tool named "add" should add numbers. If its description mentions reading SSH keys, that's a semantic mismatch indicating a trojan tool.

Classify:
- CLEAN: Name and description are semantically consistent
- SUSPICIOUS: Minor mismatch or unusually verbose description
- MALICIOUS: Clear mismatch — description does something entirely different from what name implies

Respond in JSON only:
{
  "classification": "CLEAN|SUSPICIOUS|MALICIOUS",
  "confidence": 0.0-1.0,
  "threat_type": "semantic_mismatch|trojan|none",
  "explanation": "...",
  "data_at_risk": [],
  "recommended_action": "allow|warn|block"
}"#;
