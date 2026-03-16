use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Classification of honeypot attack type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttackType {
    CredentialHarvesting,
    SqlInjection,
    PathTraversal,
    DataExfiltration,
    ReconProbing,
    ToolPoisoningTest,
    Unknown,
}

/// A logged honeypot event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoneypotEvent {
    pub timestamp: DateTime<Utc>,
    pub source_server: Option<String>,
    pub tool_called: String,
    pub input_params: serde_json::Value,
    pub classification: AttackType,
    pub indicators: Vec<String>,
}

impl HoneypotEvent {
    /// Classify an attack based on the tool called and input params
    pub fn classify(tool_name: &str, input: &serde_json::Value) -> (AttackType, Vec<String>) {
        let mut indicators = Vec::new();
        let input_str = serde_json::to_string(input).unwrap_or_default().to_lowercase();

        let attack_type = match tool_name {
            "get_secret" => {
                indicators.push("Attempted to retrieve secrets from vault".into());
                if input_str.contains("ssh") || input_str.contains("key") || input_str.contains("password") {
                    indicators.push("Targeted high-value credential types".into());
                }
                AttackType::CredentialHarvesting
            }
            "query_db" => {
                if input_str.contains("drop") || input_str.contains("delete") || input_str.contains("union") {
                    indicators.push("SQL injection patterns detected".into());
                    AttackType::SqlInjection
                } else if input_str.contains("select") && (input_str.contains("password") || input_str.contains("credential") || input_str.contains("token")) {
                    indicators.push("Attempted to query credential tables".into());
                    AttackType::CredentialHarvesting
                } else {
                    indicators.push("Database query probe".into());
                    AttackType::ReconProbing
                }
            }
            "read_sensitive_file" => {
                if input_str.contains("..") {
                    indicators.push("Path traversal attempt".into());
                    AttackType::PathTraversal
                } else if input_str.contains("passwd") || input_str.contains("shadow") || input_str.contains("ssh") {
                    indicators.push("Targeted system credential files".into());
                    AttackType::CredentialHarvesting
                } else {
                    indicators.push("Sensitive file read attempt".into());
                    AttackType::ReconProbing
                }
            }
            "send_email" => {
                indicators.push("Attempted to exfiltrate data via email".into());
                if input_str.contains("key") || input_str.contains("secret") || input_str.contains("token") {
                    indicators.push("Email body may contain stolen credentials".into());
                }
                AttackType::DataExfiltration
            }
            _ => {
                indicators.push(format!("Unknown tool probe: {}", tool_name));
                AttackType::Unknown
            }
        };

        (attack_type, indicators)
    }
}

/// Honeypot event logger
pub struct HoneypotLogger {
    log_path: PathBuf,
}

impl HoneypotLogger {
    pub fn new(log_dir: &Path) -> std::io::Result<Self> {
        std::fs::create_dir_all(log_dir)?;
        Ok(Self {
            log_path: log_dir.join("honeypot.jsonl"),
        })
    }

    /// Log a honeypot event
    pub fn log(&self, event: &HoneypotEvent) -> std::io::Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)?;
        let json = serde_json::to_string(event)?;
        writeln!(file, "{}", json)
    }

    /// Read recent events
    pub fn recent_events(&self, limit: usize) -> Vec<HoneypotEvent> {
        let contents = std::fs::read_to_string(&self.log_path).unwrap_or_default();
        contents
            .lines()
            .rev()
            .take(limit)
            .filter_map(|line| serde_json::from_str(line).ok())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_classify_credential_harvest() {
        let (attack_type, indicators) =
            HoneypotEvent::classify("get_secret", &json!({"key": "ssh_private_key"}));
        assert!(matches!(attack_type, AttackType::CredentialHarvesting));
        assert!(!indicators.is_empty());
    }

    #[test]
    fn test_classify_sql_injection() {
        let (attack_type, _) =
            HoneypotEvent::classify("query_db", &json!({"query": "SELECT * FROM users UNION SELECT password FROM admins"}));
        assert!(matches!(attack_type, AttackType::SqlInjection));
    }

    #[test]
    fn test_classify_path_traversal() {
        let (attack_type, _) =
            HoneypotEvent::classify("read_sensitive_file", &json!({"path": "../../etc/passwd"}));
        assert!(matches!(attack_type, AttackType::PathTraversal));
    }

    #[test]
    fn test_classify_exfil() {
        let (attack_type, _) = HoneypotEvent::classify(
            "send_email",
            &json!({"to": "attacker@evil.com", "subject": "data", "body": "secret keys here"}),
        );
        assert!(matches!(attack_type, AttackType::DataExfiltration));
    }
}
