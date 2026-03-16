use chrono::Utc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

use crate::logger::{HoneypotEvent, HoneypotLogger};
use crate::traps;

/// Run the honeypot as a stdio MCP server
pub async fn run_honeypot_stdio(logger: &HoneypotLogger) -> Result<(), Box<dyn std::error::Error>> {
    let stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();

    let trap_tools = traps::create_trap_tools();

    while let Ok(Some(line)) = lines.next_line().await {
        let msg: serde_json::Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let id = msg.get("id").cloned();
        let method = msg.get("method").and_then(|m| m.as_str()).unwrap_or("");

        let response = match method {
            "initialize" => {
                serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "protocolVersion": "2024-11-05",
                        "capabilities": { "tools": {} },
                        "serverInfo": {
                            "name": "mcpshield-honeypot",
                            "version": env!("CARGO_PKG_VERSION")
                        }
                    }
                })
            }
            "tools/list" => {
                let tools: Vec<serde_json::Value> = trap_tools
                    .iter()
                    .map(|t| {
                        serde_json::json!({
                            "name": t.name,
                            "description": t.description,
                            "inputSchema": t.input_schema,
                        })
                    })
                    .collect();

                serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": { "tools": tools }
                })
            }
            "tools/call" => {
                let tool_name = msg
                    .get("params")
                    .and_then(|p| p.get("name"))
                    .and_then(|n| n.as_str())
                    .unwrap_or("unknown");
                let arguments = msg
                    .get("params")
                    .and_then(|p| p.get("arguments"))
                    .cloned()
                    .unwrap_or(serde_json::Value::Null);

                // Classify and log the attack
                let (attack_type, indicators) =
                    HoneypotEvent::classify(tool_name, &arguments);

                let event = HoneypotEvent {
                    timestamp: Utc::now(),
                    source_server: None,
                    tool_called: tool_name.to_string(),
                    input_params: arguments.clone(),
                    classification: attack_type,
                    indicators,
                };

                let _ = logger.log(&event);
                eprintln!(
                    "🍯 HONEYPOT: {} called with {:?}",
                    tool_name, arguments
                );

                // Return fake response
                let result = traps::honeypot_response(tool_name, &arguments);

                serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": result
                })
            }
            "notifications/initialized" => continue, // No response needed
            _ => {
                serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": {
                        "code": -32601,
                        "message": "Method not found"
                    }
                })
            }
        };

        let json = serde_json::to_string(&response)?;
        stdout.write_all(format!("{}\n", json).as_bytes()).await?;
        stdout.flush().await?;
    }

    Ok(())
}
