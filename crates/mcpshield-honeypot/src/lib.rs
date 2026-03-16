pub mod logger;
pub mod report;
pub mod server;
pub mod traps;

use serde::{Deserialize, Serialize};

/// Honeypot configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoneypotConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_log_dir")]
    pub log_dir: String,
}

fn default_port() -> u16 { 8765 }
fn default_log_dir() -> String { "~/.mcpshield/honeypot/".into() }

impl Default for HoneypotConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            port: 8765,
            log_dir: default_log_dir(),
        }
    }
}
