pub mod analyzer;
pub mod classifier;
pub mod provider;
pub mod semantic_checker;

use std::collections::HashMap;
use std::sync::Mutex;

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use mcpshield_core::detection::severity::Severity;

/// Result of LLM analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub classification: Classification,
    pub confidence: f64,
    pub threat_type: String,
    pub explanation: String,
    pub data_at_risk: Vec<String>,
    pub recommended_action: RecommendedAction,
    pub provider_used: String,
    pub cached: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Classification {
    Clean,
    Suspicious,
    Malicious,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RecommendedAction {
    Allow,
    Warn,
    Block,
}

impl RecommendedAction {
    pub fn to_severity(&self) -> Severity {
        match self {
            RecommendedAction::Allow => Severity::Info,
            RecommendedAction::Warn => Severity::Medium,
            RecommendedAction::Block => Severity::Critical,
        }
    }
}

/// Configuration for the LLM analyzer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    #[serde(default = "default_providers")]
    pub providers: Vec<String>,
    #[serde(default = "default_trigger")]
    pub trigger: String,
    #[serde(default)]
    pub local: LocalConfig,
    #[serde(default)]
    pub anthropic: ApiConfig,
    #[serde(default)]
    pub openai: ApiConfig,
    #[serde(default)]
    pub analysis: AnalysisCacheConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalConfig {
    #[serde(default = "default_ollama_endpoint")]
    pub endpoint: String,
    #[serde(default = "default_local_model")]
    pub model: String,
    #[serde(default = "default_timeout")]
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApiConfig {
    #[serde(default)]
    pub model: String,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisCacheConfig {
    #[serde(default = "default_true")]
    pub cache_enabled: bool,
    #[serde(default = "default_cache_ttl")]
    pub cache_ttl_hours: u32,
}

fn default_providers() -> Vec<String> {
    vec!["local".into(), "anthropic".into(), "openai".into()]
}
fn default_trigger() -> String {
    "suspicious".into()
}
fn default_ollama_endpoint() -> String {
    "http://localhost:11434".into()
}
fn default_local_model() -> String {
    "llama3.2:3b".into()
}
fn default_timeout() -> u64 {
    10000
}
fn default_max_tokens() -> u32 {
    512
}
fn default_true() -> bool {
    true
}
fn default_cache_ttl() -> u32 {
    24
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            providers: default_providers(),
            trigger: default_trigger(),
            local: LocalConfig::default(),
            anthropic: ApiConfig {
                model: "claude-sonnet-4-20250514".into(),
                max_tokens: 512,
            },
            openai: ApiConfig {
                model: "gpt-4o-mini".into(),
                max_tokens: 512,
            },
            analysis: AnalysisCacheConfig::default(),
        }
    }
}

impl Default for LocalConfig {
    fn default() -> Self {
        Self {
            endpoint: default_ollama_endpoint(),
            model: default_local_model(),
            timeout_ms: default_timeout(),
        }
    }
}

impl Default for AnalysisCacheConfig {
    fn default() -> Self {
        Self {
            cache_enabled: true,
            cache_ttl_hours: 24,
        }
    }
}

/// Cache for LLM analysis results
pub struct AnalysisCache {
    cache: Mutex<HashMap<String, CachedResult>>,
    ttl: Duration,
}

struct CachedResult {
    result: AnalysisResult,
    timestamp: DateTime<Utc>,
}

impl AnalysisCache {
    pub fn new(ttl_hours: u32) -> Self {
        Self {
            cache: Mutex::new(HashMap::new()),
            ttl: Duration::hours(ttl_hours as i64),
        }
    }

    pub fn get(&self, key: &str) -> Option<AnalysisResult> {
        let cache = self.cache.lock().ok()?;
        let entry = cache.get(key)?;
        if Utc::now() - entry.timestamp < self.ttl {
            let mut result = entry.result.clone();
            result.cached = true;
            Some(result)
        } else {
            None
        }
    }

    pub fn set(&self, key: String, result: AnalysisResult) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.insert(
                key,
                CachedResult {
                    result,
                    timestamp: Utc::now(),
                },
            );
        }
    }

    pub fn cache_key(content: &str, mode: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        hasher.update(mode.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn clear(&self) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.clear();
        }
    }
}
