mod routes;
mod state;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

use state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = mcpshield_core::config::Config::load(None);
    let state = web::Data::new(AppState::new(config));

    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080u16);

    tracing::info!("MCPShield API server starting on http://127.0.0.1:{}", port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(state.clone())
            .route("/api/overview", web::get().to(routes::events::overview))
            .route("/api/events", web::get().to(routes::events::list_events))
            .route("/api/scan", web::post().to(routes::scan::trigger_scan))
            .route("/api/audit", web::get().to(routes::audit::query_audit))
            .route("/api/compliance", web::get().to(routes::compliance::get_compliance))
            .route("/api/config", web::get().to(routes::config::get_config))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
