use std::path::PathBuf;
use std::process;

use colored::Colorize;

pub fn run() {
    let config_dir = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".mcpshield");

    if let Err(e) = std::fs::create_dir_all(&config_dir) {
        eprintln!("Error creating config directory: {}", e);
        process::exit(1);
    }

    let config_path = config_dir.join("config.toml");
    if config_path.exists() {
        eprintln!("{}  Config file already exists at {}", "⚠️", config_path.display());
        return;
    }

    if let Err(e) = std::fs::write(&config_path, EXAMPLE_CONFIG) {
        eprintln!("Error writing config: {}", e);
        process::exit(1);
    }

    println!(
        "{}  Config file created at {}\n   Edit it to customize MCPShield behavior.",
        "✓".green(),
        config_path.display()
    );
}

const EXAMPLE_CONFIG: &str = r#"# MCPShield Configuration

[global]
log_level = "info"
audit_dir = "~/.mcpshield/logs"
pin_file = "~/.mcpshield/pins.json"
block_on_critical = true
alert_on_warning = true

[scan]
check_unicode = true
check_tool_poisoning = true
check_credential_leaks = true
check_cross_server = true
max_description_length = 500

[proxy]
filter_outputs = true
detect_exfiltration = true
rate_limit_per_tool = 100

[llm]
providers = ["local", "anthropic", "openai"]
trigger = "suspicious"

[llm.local]
endpoint = "http://localhost:11434"
model = "llama3.2:3b"
timeout_ms = 10000

[behavior]
enabled = true
learning_period_calls = 20
anomaly_warn_threshold = 0.6
anomaly_block_threshold = 0.85

[threat_feed]
enabled = true
sync_interval_hours = 6
submit_enabled = false

[honeypot]
enabled = false
port = 8765

[servers.filesystem]
scope = "read-only"
allowed_tools = ["read_file", "list_directory", "search_files"]

[servers.github]
scope = "read-write"
blocked_tools = ["delete_repository", "delete_file"]
require_confirmation = ["create_issue", "push", "create_pull_request"]

[servers."*"]
scope = "read-only"
block_unknown = false
"#;
