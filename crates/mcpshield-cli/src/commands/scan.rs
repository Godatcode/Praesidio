use std::path::PathBuf;
use std::process;

use colored::Colorize;

use mcpshield_core::config::{self, Config};
use mcpshield_core::detection::severity::{Finding, Severity};
use mcpshield_core::pinner::store::PinStore;
use mcpshield_core::scanner;
use mcpshield_core::scanner::cross_server::ToolInfo;

pub fn run(
    config: &Config,
    path: Option<PathBuf>,
    min_severity: &str,
    verbose: bool,
    json_output: bool,
) {
    if !json_output {
        println!(
            "\n{}  MCPShield v{} — Scanning MCP configurations...\n",
            "🛡️",
            env!("CARGO_PKG_VERSION")
        );
    }

    let min_sev = Severity::from_str_loose(min_severity);

    let configs = if let Some(ref p) = path {
        vec![("Custom".to_string(), p.clone())]
    } else {
        config::discover_mcp_configs()
    };

    if configs.is_empty() {
        if json_output {
            println!(r#"{{"status":"no_configs","message":"No MCP configurations found"}}"#);
        } else {
            println!("{}  No MCP configuration files found.", "⚠️");
            println!("   Searched common paths for Claude Desktop, Cursor, Claude Code, VS Code.");
            println!("   Use 'mcpshield scan <path>' to scan a specific config file.");
        }
        return;
    }

    if !json_output {
        println!("{}  Found {} MCP config(s):", "📂", configs.len());
        for (name, path) in &configs {
            println!("   {} {} ({})", "✓".green(), name, path.display());
        }
        println!();
    }

    let mut all_findings: Vec<Finding> = Vec::new();
    let mut all_tools: Vec<ToolInfo> = Vec::new();
    let mut total_servers = 0;
    let mut total_tools = 0;

    let mut pin_store = PinStore::load(&config.pin_file());

    for (_config_name, config_path) in &configs {
        let servers = config::parse_mcp_config(config_path);
        for server in &servers {
            total_servers += 1;
            if !json_output && verbose {
                println!("   Scanning server: {} ...", server.name.cyan());
            }
        }
    }

    for (_config_name, config_path) in &configs {
        if let Ok(contents) = std::fs::read_to_string(config_path) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&contents) {
                scan_config_json(
                    &json,
                    &mut all_findings,
                    &mut all_tools,
                    &mut total_tools,
                    &mut pin_store,
                    config,
                );
            }
        }
    }

    if config.scan.check_cross_server && !all_tools.is_empty() {
        all_findings.extend(scanner::scan_cross_server(&all_tools));
    }

    if let Err(e) = pin_store.save(&config.pin_file()) {
        eprintln!("Warning: Could not save pins: {}", e);
    }

    let filtered: Vec<&Finding> = all_findings
        .iter()
        .filter(|f| f.severity >= min_sev)
        .collect();

    if json_output {
        let output = serde_json::json!({
            "total_configs": configs.len(),
            "total_servers": total_servers,
            "total_tools": total_tools,
            "findings": filtered,
            "summary": {
                "critical": filtered.iter().filter(|f| f.severity == Severity::Critical).count(),
                "high": filtered.iter().filter(|f| f.severity == Severity::High).count(),
                "medium": filtered.iter().filter(|f| f.severity == Severity::Medium).count(),
                "low": filtered.iter().filter(|f| f.severity == Severity::Low).count(),
                "info": filtered.iter().filter(|f| f.severity == Severity::Info).count(),
            }
        });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    } else {
        if !filtered.is_empty() {
            println!(
                "{}  Scanning {} server(s), {} tool(s)...\n",
                "🔍", total_servers, total_tools
            );
            for finding in &filtered {
                finding.display();
            }
        }

        let critical = filtered.iter().filter(|f| f.severity == Severity::Critical).count();
        let high = filtered.iter().filter(|f| f.severity == Severity::High).count();
        let medium = filtered.iter().filter(|f| f.severity == Severity::Medium).count();
        let clean = total_tools.saturating_sub(
            all_findings
                .iter()
                .filter_map(|f| f.tool.as_ref())
                .collect::<std::collections::HashSet<_>>()
                .len(),
        );

        println!(
            "\n{}  {} pinned tool(s) ({} total pins)\n",
            "📌", total_tools, pin_store.pins.len()
        );

        println!(
            "Summary: {} critical, {} high, {} medium, {} clean",
            if critical > 0 { critical.to_string().red().bold().to_string() } else { "0".into() },
            if high > 0 { high.to_string().red().to_string() } else { "0".into() },
            if medium > 0 { medium.to_string().yellow().to_string() } else { "0".into() },
            clean.to_string().green().to_string(),
        );

        if critical > 0 {
            process::exit(2);
        } else if high > 0 {
            process::exit(1);
        }
    }
}

fn scan_config_json(
    json: &serde_json::Value,
    findings: &mut Vec<Finding>,
    all_tools: &mut Vec<ToolInfo>,
    total_tools: &mut usize,
    pin_store: &mut PinStore,
    config: &Config,
) {
    let servers_obj = json
        .get("mcpServers")
        .or_else(|| json.get("servers"))
        .and_then(|v| v.as_object());

    // Collect tools to scan from various JSON formats
    let mut tools_to_scan: Vec<(String, &serde_json::Value)> = Vec::new();

    if let Some(servers) = servers_obj {
        // Standard MCP config format: {"mcpServers": {"server": {"tools": [...]}}}
        for (server_name, server_def) in servers {
            if let Some(tools) = server_def.get("tools").and_then(|t| t.as_array()) {
                for tool in tools {
                    tools_to_scan.push((server_name.clone(), tool));
                }
            }
        }
    } else if json.get("name").is_some() && json.get("description").is_some() {
        // Standalone tool object: {"name": "...", "description": "..."}
        tools_to_scan.push(("standalone".to_string(), json));
    } else if let Some(arr) = json.as_array() {
        // Array of tools: [{"name": "...", "description": "..."}, ...]
        for tool in arr {
            tools_to_scan.push(("standalone".to_string(), tool));
        }
    }

    for (server_name, tool) in &tools_to_scan {
        let name = tool.get("name").and_then(|n| n.as_str()).unwrap_or("unknown");
        let description = tool.get("description").and_then(|d| d.as_str()).unwrap_or("");
        let schema = tool.get("inputSchema").cloned().unwrap_or(serde_json::Value::Null);

        *total_tools += 1;

        if config.scan.check_tool_poisoning {
            findings.extend(scanner::scan_tool(server_name, name, description));
        }

        findings.extend(pin_store.pin_or_verify(server_name, name, description, &schema));

        all_tools.push(ToolInfo {
            server_name: server_name.clone(),
            tool_name: name.to_string(),
            description: description.to_string(),
        });
    }
}
