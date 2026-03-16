use actix_web::{web, HttpResponse};

use crate::state::AppState;

pub async fn get_config(state: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().json(&state.config)
}
