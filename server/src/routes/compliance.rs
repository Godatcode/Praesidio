use actix_web::HttpResponse;

pub async fn get_compliance() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "mcp_top10_coverage": 10,
        "agentic_top10_coverage": 10,
        "total_risks": 20,
        "covered": 20,
    }))
}
