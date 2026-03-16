pub mod event;

use chrono::{Duration, Utc};
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};

use crate::detection::severity::Severity;
use event::AuditEvent;

pub struct AuditLogger {
    log_path: PathBuf,
}

impl AuditLogger {
    pub fn new(audit_dir: &Path) -> std::io::Result<Self> {
        std::fs::create_dir_all(audit_dir)?;
        let log_path = audit_dir.join("audit.jsonl");
        Ok(Self { log_path })
    }

    /// Append an audit event to the log
    pub fn log(&self, event: &AuditEvent) -> std::io::Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)?;

        let json = serde_json::to_string(event)?;
        writeln!(file, "{}", json)?;
        Ok(())
    }

    /// Query audit events with filters
    pub fn query(
        &self,
        last_duration: Option<&str>,
        min_severity: Option<Severity>,
    ) -> Vec<AuditEvent> {
        let mut events = Vec::new();

        let file = match std::fs::File::open(&self.log_path) {
            Ok(f) => f,
            Err(_) => return events,
        };

        let cutoff = last_duration.and_then(|d| parse_duration(d)).map(|dur| Utc::now() - dur);

        let reader = std::io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                if let Ok(event) = serde_json::from_str::<AuditEvent>(&line) {
                    // Apply time filter
                    if let Some(ref cutoff) = cutoff {
                        if event.timestamp < *cutoff {
                            continue;
                        }
                    }

                    // Apply severity filter
                    if let Some(min) = min_severity {
                        if event.severity < min {
                            continue;
                        }
                    }

                    events.push(event);
                }
            }
        }

        events
    }
}

/// Parse a duration string like "24h", "7d", "30m"
fn parse_duration(s: &str) -> Option<Duration> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    let (num_str, unit) = s.split_at(s.len() - 1);
    let num: i64 = num_str.parse().ok()?;

    match unit {
        "m" => Duration::try_minutes(num),
        "h" => Duration::try_hours(num),
        "d" => Duration::try_days(num),
        "w" => Duration::try_weeks(num),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration() {
        assert!(parse_duration("24h").is_some());
        assert!(parse_duration("7d").is_some());
        assert!(parse_duration("30m").is_some());
        assert!(parse_duration("2w").is_some());
        assert!(parse_duration("").is_none());
        assert!(parse_duration("xyz").is_none());
    }
}
