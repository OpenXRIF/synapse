extern crate actix_web;

use actix_web::{web, HttpResponse};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/health").route("", web::get().to(health_check)));
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "Healthy"
    }))
}
