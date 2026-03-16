use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::detection::severity::Severity;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    Scan,
    Block,
    Alert,
    Allow,
    PinCreated,
    PinChanged,
    ConfigLoaded,
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::Scan => write!(f, "scan"),
            EventType::Block => write!(f, "block"),
            EventType::Alert => write!(f, "alert"),
            EventType::Allow => write!(f, "allow"),
            EventType::PinCreated => write!(f, "pin_created"),
            EventType::PinChanged => write!(f, "pin_changed"),
            EventType::ConfigLoaded => write!(f, "config_loaded"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub timestamp: DateTime<Utc>,
    pub event_type: EventType,
    pub severity: Severity,
    pub server: String,
    pub tool: Option<String>,
    pub description: String,
    pub details: serde_json::Value,
}

impl AuditEvent {
    pub fn new(
        event_type: EventType,
        severity: Severity,
        server: &str,
        tool: Option<&str>,
        description: &str,
    ) -> Self {
        Self {
            timestamp: Utc::now(),
            event_type,
            severity,
            server: server.to_string(),
            tool: tool.map(String::from),
            description: description.to_string(),
            details: serde_json::Value::Null,
        }
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = details;
        self
    }
}
