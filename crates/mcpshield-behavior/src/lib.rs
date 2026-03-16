pub mod anomaly;
pub mod baseline;
pub mod features;
pub mod profile;
pub mod store;

use serde::{Deserialize, Serialize};

/// Configuration for behavioral fingerprinting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_learning_period")]
    pub learning_period_calls: u64,
    #[serde(default = "default_warn_threshold")]
    pub anomaly_warn_threshold: f64,
    #[serde(default = "default_block_threshold")]
    pub anomaly_block_threshold: f64,
    #[serde(default = "default_profile_dir")]
    pub profile_dir: String,
    #[serde(default)]
    pub sensitivity: SensitivityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitivityConfig {
    #[serde(default = "default_high")]
    pub output_size: String,
    #[serde(default = "default_high")]
    pub entropy: String,
    #[serde(default = "default_medium")]
    pub response_time: String,
    #[serde(default = "default_medium")]
    pub call_frequency: String,
    #[serde(default = "default_high")]
    pub base64_density: String,
}

fn default_true() -> bool { true }
fn default_learning_period() -> u64 { 20 }
fn default_warn_threshold() -> f64 { 0.6 }
fn default_block_threshold() -> f64 { 0.85 }
fn default_profile_dir() -> String { "~/.mcpshield/profiles/".into() }
fn default_high() -> String { "high".into() }
fn default_medium() -> String { "medium".into() }

impl Default for BehaviorConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            learning_period_calls: 20,
            anomaly_warn_threshold: 0.6,
            anomaly_block_threshold: 0.85,
            profile_dir: default_profile_dir(),
            sensitivity: SensitivityConfig::default(),
        }
    }
}

impl Default for SensitivityConfig {
    fn default() -> Self {
        Self {
            output_size: "high".into(),
            entropy: "high".into(),
            response_time: "medium".into(),
            call_frequency: "medium".into(),
            base64_density: "high".into(),
        }
    }
}

impl SensitivityConfig {
    /// Convert sensitivity label to weight multiplier
    pub fn weight(&self, feature: &str) -> f64 {
        let level = match feature {
            "output_size" => &self.output_size,
            "entropy" => &self.entropy,
            "response_time" => &self.response_time,
            "call_frequency" => &self.call_frequency,
            "base64_density" => &self.base64_density,
            _ => return 1.0,
        };
        match level.as_str() {
            "high" => 1.5,
            "medium" => 1.0,
            "low" => 0.5,
            _ => 1.0,
        }
    }
}
