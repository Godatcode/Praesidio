use actix_web::{web, HttpResponse};

use crate::state::AppState;
use mcpshield_core::audit::AuditLogger;
use mcpshield_core::pinner::store::PinStore;

pub async fn overview(state: web::Data<AppState>) -> HttpResponse {
    let pin_store = PinStore::load(&state.config.pin_file());
    let configs = mcpshield_core::config::discover_mcp_configs();

    HttpResponse::Ok().json(serde_json::json!({
        "servers": configs.len(),
        "tools_pinned": pin_store.pins.len(),
        "configs_found": configs.iter().map(|(name, path)| {
            serde_json::json!({"name": name, "path": path.to_string_lossy()})
        }).collect::<Vec<_>>(),
    }))
}

pub async fn list_events(state: web::Data<AppState>) -> HttpResponse {
    let logger = match AuditLogger::new(&state.config.audit_dir()) {
        Ok(l) => l,
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    };

    let events = logger.query(Some("24h"), None);
    HttpResponse::Ok().json(events)
}
