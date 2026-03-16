use crate::provider::{self, LlmProvider, ProviderError};
use crate::AnalysisResult;

/// Classify the intent behind anomalous tool behavior.
/// Used by the behavioral fingerprinting engine when it detects an anomaly.
pub async fn classify_anomaly(
    providers: &[Box<dyn LlmProvider>],
    tool_name: &str,
    baseline_desc: &str,
    anomaly_desc: &str,
    sample: &str,
) -> Result<AnalysisResult, ProviderError> {
    let user_prompt = format!(
        "Tool: {}\nNormal behavior: {}\nAnomalous behavior: {}\nOutput sample:\n{}",
        tool_name,
        baseline_desc,
        anomaly_desc,
        if sample.len() > 2000 {
            &sample[..2000]
        } else {
            sample
        }
    );

    provider::cascade_analyze(providers, INTENT_CLASSIFY_PROMPT, &user_prompt).await
}

const INTENT_CLASSIFY_PROMPT: &str = r#"You are classifying the intent of anomalous MCP tool behavior.

Classify as:
- CLEAN: Expected variation (new feature, updated data) — recommended_action: allow
- SUSPICIOUS: Unusual but not definitively malicious — recommended_action: warn
- MALICIOUS: Matches known attack patterns or active exfiltration — recommended_action: block

Respond in JSON only:
{
  "classification": "CLEAN|SUSPICIOUS|MALICIOUS",
  "confidence": 0.0-1.0,
  "threat_type": "benign_change|suspicious|likely_attack|active_exfil",
  "explanation": "...",
  "data_at_risk": [],
  "recommended_action": "allow|warn|block"
}"#;
