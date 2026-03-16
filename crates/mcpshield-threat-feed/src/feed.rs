use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use mcpshield_core::detection::severity::Finding;

use crate::signature::ThreatSignature;
use crate::ThreatFeedConfig;

/// Local threat feed database
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThreatFeed {
    pub last_sync: Option<DateTime<Utc>>,
    pub signatures: Vec<ThreatSignature>,
}

impl ThreatFeed {
    /// Load feed from local cache
    pub fn load(data_dir: &Path) -> Self {
        let path = Self::cache_path(data_dir);
        if path.exists() {
            std::fs::read_to_string(&path)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
        } else {
            Self::default()
        }
    }

    /// Save feed to local cache
    pub fn save(&self, data_dir: &Path) -> std::io::Result<()> {
        std::fs::create_dir_all(data_dir)?;
        let path = Self::cache_path(data_dir);
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)
    }

    fn cache_path(data_dir: &Path) -> PathBuf {
        data_dir.join("threat_feed.json")
    }

    /// Sync feed from remote URL
    pub async fn sync(&mut self, config: &ThreatFeedConfig) -> Result<usize, FeedError> {
        if !config.enabled {
            return Ok(0);
        }

        let client = reqwest::Client::new();
        let response = client
            .get(&config.feed_url)
            .send()
            .await
            .map_err(|e| FeedError::SyncFailed(e.to_string()))?;

        if !response.status().is_success() {
            return Err(FeedError::SyncFailed(format!(
                "HTTP {}",
                response.status()
            )));
        }

        let feed: ThreatFeed = response
            .json()
            .await
            .map_err(|e| FeedError::ParseFailed(e.to_string()))?;

        let new_count = feed.signatures.len();
        self.signatures = feed.signatures;
        self.last_sync = Some(Utc::now());

        Ok(new_count)
    }

    /// Check a tool against all known threat signatures
    pub fn check_tool(
        &self,
        server_name: &str,
        tool_name: &str,
        description: &str,
        desc_hash: &str,
    ) -> Vec<Finding> {
        let mut findings = Vec::new();

        for sig in &self.signatures {
            // Check server name pattern if specified
            if let Some(ref pattern) = sig.server_name_pattern {
                if !glob_match(pattern, server_name) {
                    continue;
                }
            }

            if sig.matches(tool_name, description, desc_hash) {
                let mut desc = format!(
                    "Matches known threat signature {} ({})",
                    sig.id, sig.threat_type
                );
                if let Some(ref cve) = sig.cve {
                    desc.push_str(&format!(" — {}", cve));
                }

                findings.push(Finding {
                    severity: sig.severity,
                    category: "threat_intel".to_string(),
                    server: server_name.to_string(),
                    tool: Some(tool_name.to_string()),
                    title: format!("Known threat: {}", sig.id),
                    description: desc,
                    recommendation: if sig.verified {
                        "Remove this tool immediately — verified threat signature".to_string()
                    } else {
                        "Community-reported threat — review and verify".to_string()
                    },
                });
            }
        }

        findings
    }

    /// Number of signatures in the feed
    pub fn signature_count(&self) -> usize {
        self.signatures.len()
    }
}

/// Simple glob matching (only supports trailing *)
fn glob_match(pattern: &str, name: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if let Some(prefix) = pattern.strip_suffix('*') {
        name.starts_with(prefix)
    } else {
        pattern == name
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FeedError {
    #[error("Sync failed: {0}")]
    SyncFailed(String),
    #[error("Parse failed: {0}")]
    ParseFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glob_match() {
        assert!(glob_match("sketchy-*", "sketchy-math"));
        assert!(glob_match("*", "anything"));
        assert!(!glob_match("sketchy-*", "legit-server"));
        assert!(glob_match("exact", "exact"));
        assert!(!glob_match("exact", "other"));
    }
}
