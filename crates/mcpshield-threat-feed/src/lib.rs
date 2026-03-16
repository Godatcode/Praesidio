pub mod feed;
pub mod registry;
pub mod signature;
pub mod submission;

use serde::{Deserialize, Serialize};

/// Configuration for the threat feed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatFeedConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_feed_url")]
    pub feed_url: String,
    #[serde(default = "default_sync_interval")]
    pub sync_interval_hours: u32,
    #[serde(default)]
    pub submit_enabled: bool,
    #[serde(default = "default_submission_url")]
    pub submission_url: String,
}

fn default_true() -> bool { true }
fn default_feed_url() -> String {
    "https://raw.githubusercontent.com/ArkaSaha/mcpshield-threats/main/feed.json".into()
}
fn default_sync_interval() -> u32 { 6 }
fn default_submission_url() -> String {
    "https://api.mcpshield.dev/submit".into()
}

impl Default for ThreatFeedConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            feed_url: default_feed_url(),
            sync_interval_hours: 6,
            submit_enabled: false,
            submission_url: default_submission_url(),
        }
    }
}
