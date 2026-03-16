use actix_web::{web, HttpResponse};

use crate::state::AppState;
use mcpshield_core::config;
use mcpshield_core::detection::severity::Severity;
use mcpshield_core::pinner::store::PinStore;
use mcpshield_core::scanner;
use mcpshield_core::scanner::cross_server::ToolInfo;

pub async fn trigger_scan(state: web::Data<AppState>) -> HttpResponse {
    let configs = config::discover_mcp_configs();
    let mut all_findings = Vec::new();
    let mut all_tools = Vec::new();
    let mut total_tools = 0usize;
    let mut pin_store = PinStore::load(&state.config.pin_file());

    for (_name, config_path) in &configs {
        if let Ok(contents) = std::fs::read_to_string(config_path) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&contents) {
                let servers_obj = json.get("mcpServers").or_else(|| json.get("servers")).and_then(|v| v.as_object());
                if let Some(servers) = servers_obj {
                    for (server_name, server_def) in servers {
                        if let Some(tools) = server_def.get("tools").and_then(|t| t.as_array()) {
                            for tool in tools {
                                let name = tool.get("name").and_then(|n| n.as_str()).unwrap_or("unknown");
                                let desc = tool.get("description").and_then(|d| d.as_str()).unwrap_or("");
                                let schema = tool.get("inputSchema").cloned().unwrap_or_default();
                                total_tools += 1;
                                all_findings.extend(scanner::scan_tool(server_name, name, desc));
                                all_findings.extend(pin_store.pin_or_verify(server_name, name, desc, &schema));
                                all_tools.push(ToolInfo { server_name: server_name.clone(), tool_name: name.into(), description: desc.into() });
                            }
                        }
                    }
                }
            }
        }
    }

    if !all_tools.is_empty() {
        all_findings.extend(scanner::scan_cross_server(&all_tools));
    }
    let _ = pin_store.save(&state.config.pin_file());

    HttpResponse::Ok().json(serde_json::json!({
        "total_tools": total_tools,
        "findings": all_findings,
        "summary": {
            "critical": all_findings.iter().filter(|f| f.severity == Severity::Critical).count(),
            "high": all_findings.iter().filter(|f| f.severity == Severity::High).count(),
            "medium": all_findings.iter().filter(|f| f.severity == Severity::Medium).count(),
        }
    }))
}
