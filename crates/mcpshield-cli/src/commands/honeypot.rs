use mcpshield_core::config::Config;
use mcpshield_honeypot::logger::HoneypotLogger;
use mcpshield_honeypot::report::HoneypotReport;

use crate::HoneypotAction;

pub fn run(_config: &Config, action: HoneypotAction) {
    let log_dir = mcpshield_core::config::Config::expand_path("~/.mcpshield/honeypot");

    match action {
        HoneypotAction::Start => {
            println!("{}  Starting MCPShield honeypot server...", "🍯");
            println!("   This server mimics vulnerable tools to detect attackers.");
            println!("   Logs: {}\n", log_dir.display());

            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let logger = HoneypotLogger::new(&log_dir)
                    .expect("Failed to create honeypot logger");
                if let Err(e) = mcpshield_honeypot::server::run_honeypot_stdio(&logger).await {
                    eprintln!("Honeypot error: {}", e);
                }
            });
        }
        HoneypotAction::Attacks { last: _ } => {
            let logger = match HoneypotLogger::new(&log_dir) {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };

            let events = logger.recent_events(50);
            if events.is_empty() {
                println!("No honeypot events recorded yet.");
                return;
            }

            println!("{}  Recent honeypot attacks ({}):\n", "🍯", events.len());
            for event in &events {
                println!(
                    "   [{}] {:?} — {} ({})",
                    event.timestamp.format("%Y-%m-%d %H:%M:%S"),
                    event.classification,
                    event.tool_called,
                    event.indicators.join(", ")
                );
            }
        }
        HoneypotAction::Report { format } => {
            let logger = match HoneypotLogger::new(&log_dir) {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };

            let events = logger.recent_events(1000);
            let report = HoneypotReport::from_events(&events);

            if format == "json" {
                println!("{}", serde_json::to_string_pretty(&report).unwrap());
            } else {
                println!("{}  Honeypot Report\n", "🍯");
                println!("   Total events: {}", report.total_events);
                println!("   By type:");
                for (t, count) in &report.by_type {
                    println!("     {}: {}", t, count);
                }
                println!("   By tool:");
                for (t, count) in &report.by_tool {
                    println!("     {}: {}", t, count);
                }
            }
        }
    }
}
