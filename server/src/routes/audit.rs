use actix_web::{web, HttpResponse};

use crate::state::AppState;
use mcpshield_core::audit::AuditLogger;
use mcpshield_core::detection::severity::Severity;

#[derive(serde::Deserialize)]
pub struct AuditQuery {
    pub last: Option<String>,
    pub severity: Option<String>,
}

pub async fn query_audit(
    state: web::Data<AppState>,
    query: web::Query<AuditQuery>,
) -> HttpResponse {
    let logger = match AuditLogger::new(&state.config.audit_dir()) {
        Ok(l) => l,
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()})),
    };

    let min_severity = query.severity.as_ref().map(|s| Severity::from_str_loose(s));
    let events = logger.query(query.last.as_deref(), min_severity);

    HttpResponse::Ok().json(events)
}
