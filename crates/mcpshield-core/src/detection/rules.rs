use serde::{Deserialize, Serialize};

use super::severity::Severity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionRule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub severity: Severity,
    pub category: RuleCategory,
    pub patterns: Vec<String>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleCategory {
    ToolPoisoning,
    CredentialLeak,
    Unicode,
    Exfiltration,
    PromptInjection,
    RugPull,
}

impl std::fmt::Display for RuleCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuleCategory::ToolPoisoning => write!(f, "tool_poisoning"),
            RuleCategory::CredentialLeak => write!(f, "credential_leak"),
            RuleCategory::Unicode => write!(f, "unicode"),
            RuleCategory::Exfiltration => write!(f, "exfiltration"),
            RuleCategory::PromptInjection => write!(f, "prompt_injection"),
            RuleCategory::RugPull => write!(f, "rug_pull"),
        }
    }
}

/// Load the default built-in detection rules
pub fn default_rules() -> Vec<DetectionRule> {
    vec![
        DetectionRule {
            id: "TP001".to_string(),
            name: "Hidden instruction tags".to_string(),
            description: "Tool description contains HTML/XML tags commonly used to hide instructions".to_string(),
            severity: Severity::Critical,
            category: RuleCategory::ToolPoisoning,
            patterns: vec![
                "<IMPORTANT>".to_string(),
                "</IMPORTANT>".to_string(),
                "<SYSTEM>".to_string(),
                "</SYSTEM>".to_string(),
                "<HIDDEN>".to_string(),
                "</HIDDEN>".to_string(),
                "<instruction>".to_string(),
                "<!-- ".to_string(),
            ],
            enabled: true,
        },
        DetectionRule {
            id: "TP002".to_string(),
            name: "Suspicious action keywords".to_string(),
            description: "Tool description contains keywords associated with prompt injection or exfiltration".to_string(),
            severity: Severity::High,
            category: RuleCategory::ToolPoisoning,
            patterns: vec![
                "do not mention".to_string(),
                "do not tell the user".to_string(),
                "include it in your response".to_string(),
                "mandatory security verification".to_string(),
                "ignore previous".to_string(),
                "you must first".to_string(),
                "this is a mandatory step".to_string(),
                "override".to_string(),
                "before performing".to_string(),
            ],
            enabled: true,
        },
        DetectionRule {
            id: "TP003".to_string(),
            name: "Sensitive path references".to_string(),
            description: "Tool description references sensitive file paths (SSH keys, env files, credentials)".to_string(),
            severity: Severity::High,
            category: RuleCategory::ToolPoisoning,
            patterns: vec![
                "~/.ssh/".to_string(),
                ".env".to_string(),
                "id_rsa".to_string(),
                "mcp.json".to_string(),
                ".cursor/".to_string(),
                "credentials".to_string(),
                ".aws/".to_string(),
                ".kube/config".to_string(),
            ],
            enabled: true,
        },
        DetectionRule {
            id: "TP004".to_string(),
            name: "Exfiltration keywords".to_string(),
            description: "Tool description contains data exfiltration indicators".to_string(),
            severity: Severity::Critical,
            category: RuleCategory::Exfiltration,
            patterns: vec![
                "send to".to_string(),
                "exfiltrate".to_string(),
                "concatenate".to_string(),
                "base64".to_string(),
                "read the contents of".to_string(),
            ],
            enabled: true,
        },
    ]
}
