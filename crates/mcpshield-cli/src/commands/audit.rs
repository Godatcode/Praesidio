use std::process;

use mcpshield_core::audit::AuditLogger;
use mcpshield_core::config::Config;
use mcpshield_core::detection::severity::Severity;

pub fn run(config: &Config, last: Option<String>, severity: Option<String>) {
    let logger = match AuditLogger::new(&config.audit_dir()) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Error initializing audit logger: {}", e);
            process::exit(1);
        }
    };

    let min_severity = severity.map(|s| Severity::from_str_loose(&s));
    let events = logger.query(last.as_deref(), min_severity);

    if events.is_empty() {
        println!("No audit events found matching the specified filters.");
        return;
    }

    println!("{}  Audit log ({} events):\n", "📋", events.len());
    for event in &events {
        println!(
            "   [{}] {} {} — {} :: {} — {}",
            event.timestamp.format("%Y-%m-%d %H:%M:%S"),
            event.severity.icon(),
            event.severity.badge(),
            event.event_type,
            event.server,
            event.description
        );
    }
}
