use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl Severity {
    pub fn badge(&self) -> String {
        match self {
            Severity::Info => "INFO".dimmed().to_string(),
            Severity::Low => "LOW".blue().to_string(),
            Severity::Medium => "MEDIUM".yellow().to_string(),
            Severity::High => "HIGH".red().to_string(),
            Severity::Critical => "CRITICAL".red().bold().to_string(),
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Severity::Info => "ℹ️",
            Severity::Low => "🔵",
            Severity::Medium => "⚠️",
            Severity::High => "🔴",
            Severity::Critical => "🚨",
        }
    }

    pub fn from_str_loose(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "critical" => Severity::Critical,
            "high" => Severity::High,
            "medium" | "med" => Severity::Medium,
            "low" => Severity::Low,
            _ => Severity::Info,
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Info => write!(f, "info"),
            Severity::Low => write!(f, "low"),
            Severity::Medium => write!(f, "medium"),
            Severity::High => write!(f, "high"),
            Severity::Critical => write!(f, "critical"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub severity: Severity,
    pub category: String,
    pub server: String,
    pub tool: Option<String>,
    pub title: String,
    pub description: String,
    pub recommendation: String,
}

impl Finding {
    pub fn display(&self) {
        let tool_str = self
            .tool
            .as_deref()
            .map(|t| format!(" → {}", t))
            .unwrap_or_default();

        println!(
            "\n{} {}: {}",
            self.severity.icon(),
            self.severity.badge(),
            self.title.bold()
        );
        println!(
            "   Server: {}{}",
            self.server.cyan(),
            tool_str.cyan()
        );
        println!("   {}", self.description);
        println!("   Action: {}", self.recommendation.yellow());
    }

    pub fn display_json(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            println!("{}", json);
        }
    }
}
