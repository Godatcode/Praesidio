use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use mcpshield_core::detection::severity::Severity;

/// An anonymized threat submission (opt-in)
/// ONLY tool metadata — never user data, file contents, or tool outputs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatSubmission {
    pub tool_description_hash: String,
    pub server_package_name: String,
    pub detection_type: String,
    pub severity: Severity,
    pub mcpshield_version: String,
    pub timestamp: DateTime<Utc>,
}

impl ThreatSubmission {
    pub fn new(
        desc_hash: &str,
        server_name: &str,
        detection_type: &str,
        severity: Severity,
    ) -> Self {
        Self {
            tool_description_hash: desc_hash.to_string(),
            server_package_name: server_name.to_string(),
            detection_type: detection_type.to_string(),
            severity,
            mcpshield_version: env!("CARGO_PKG_VERSION").to_string(),
            timestamp: Utc::now(),
        }
    }
}

/// Submit a threat finding to the community feed (opt-in only)
pub async fn submit_threat(
    submission: &ThreatSubmission,
    url: &str,
) -> Result<(), SubmissionError> {
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .json(submission)
        .send()
        .await
        .map_err(|e| SubmissionError::Failed(e.to_string()))?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(SubmissionError::Failed(format!(
            "HTTP {}",
            response.status()
        )))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SubmissionError {
    #[error("Submission failed: {0}")]
    Failed(String),
}
