use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;

use crate::audit::event::{AuditEvent, EventType};
use crate::audit::AuditLogger;
use crate::config::Config;
use crate::detection::severity::Severity;
use crate::filter;
use crate::permissions::enforcer;
use crate::scanner;

use super::message::JsonRpcMessage;

/// Start a stdio proxy between the client and a child MCP server process
pub async fn start_stdio_proxy(
    server_cmd: &str,
    config: &Config,
    audit_logger: &AuditLogger,
    server_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Parse the command
    let parts: Vec<&str> = server_cmd.split_whitespace().collect();
    if parts.is_empty() {
        return Err("Empty server command".into());
    }

    let mut child = Command::new(parts[0])
        .args(&parts[1..])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;

    let child_stdin = child.stdin.take().expect("Failed to open child stdin");
    let child_stdout = child.stdout.take().expect("Failed to open child stdout");

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let config_clone = config.clone();
    let server_name_owned = server_name.to_string();

    // Forward client stdin → child stdin (intercepting requests)
    let client_to_server = {
        let config = config_clone.clone();
        let server_name = server_name_owned.clone();
        tokio::spawn(async move {
            let reader = BufReader::new(stdin);
            let mut writer = child_stdin;
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                // Try to parse as JSON-RPC
                if let Ok(msg) = serde_json::from_str::<JsonRpcMessage>(&line) {
                    // Check permissions for tool calls
                    if msg.is_tool_call() {
                        if let Some(tool_name) = msg.tool_call_name() {
                            let server_config = config.servers.get(&server_name);
                            let (result, _finding) = enforcer::check_permission(
                                &server_name,
                                tool_name,
                                server_config,
                            );

                            if let enforcer::PermissionResult::Blocked(reason) = result {
                                // Send error response back to client
                                if let Some(id) = &msg.id {
                                    let error = JsonRpcMessage::error_response(
                                        id.clone(),
                                        -32600,
                                        &format!("MCPShield: {}", reason),
                                    );
                                    if let Ok(json) = serde_json::to_string(&error) {
                                        let mut stdout = tokio::io::stdout();
                                        let _ = stdout
                                            .write_all(format!("{}\n", json).as_bytes())
                                            .await;
                                        let _ = stdout.flush().await;
                                    }
                                }
                                continue; // Don't forward to server
                            }
                        }
                    }
                }

                // Forward to server
                if writer
                    .write_all(format!("{}\n", line).as_bytes())
                    .await
                    .is_err()
                {
                    break;
                }
                if writer.flush().await.is_err() {
                    break;
                }
            }
        })
    };

    // Forward child stdout → client stdout (intercepting responses)
    let server_to_client = {
        let config = config_clone;
        let server_name = server_name_owned;
        tokio::spawn(async move {
            let reader = BufReader::new(child_stdout);
            let mut writer = stdout;
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                let mut should_forward = true;
                let mut modified_line = line.clone();

                // Try to parse as JSON-RPC
                if let Ok(msg) = serde_json::from_str::<JsonRpcMessage>(&line) {
                    // Scan tools/list responses
                    if msg.is_tools_list_response() {
                        let tools = msg.extract_tools();
                        for tool in &tools {
                            let findings = scanner::scan_tool(
                                &server_name,
                                &tool.name,
                                &tool.description,
                            );
                            for finding in &findings {
                                finding.display();
                                if finding.severity >= Severity::Critical
                                    && config.global.block_on_critical
                                {
                                    eprintln!(
                                        "\n{} Blocking critical finding for tool '{}'",
                                        "🛡️", tool.name
                                    );
                                }
                            }
                        }
                    }

                    // Filter tool results
                    if msg.is_response() {
                        if let Some(text) = msg.extract_tool_result_text() {
                            if config.proxy.filter_outputs {
                                let findings = filter::filter_output(
                                    &server_name,
                                    "unknown",
                                    &text,
                                );
                                for finding in &findings {
                                    finding.display();
                                    if finding.severity >= Severity::Critical
                                        && config.global.block_on_critical
                                    {
                                        should_forward = false;
                                        // Send sanitized error instead
                                        if let Some(id) = &msg.id {
                                            let error = JsonRpcMessage::error_response(
                                                id.clone(),
                                                -32600,
                                                "MCPShield: Output blocked due to security findings",
                                            );
                                            if let Ok(json) = serde_json::to_string(&error) {
                                                modified_line = json;
                                                should_forward = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if should_forward {
                    if writer
                        .write_all(format!("{}\n", modified_line).as_bytes())
                        .await
                        .is_err()
                    {
                        break;
                    }
                    if writer.flush().await.is_err() {
                        break;
                    }
                }
            }
        })
    };

    // Log proxy start
    let _ = audit_logger.log(&AuditEvent::new(
        EventType::Allow,
        Severity::Info,
        server_name,
        None,
        &format!("Proxy started for command: {}", server_cmd),
    ));

    // Wait for either task to complete
    tokio::select! {
        _ = client_to_server => {},
        _ = server_to_client => {},
    }

    let _ = child.kill().await;
    Ok(())
}
