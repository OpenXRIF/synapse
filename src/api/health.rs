use actix_web::{web, HttpResponse, Scope};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/health").route("", web::get().to(health_check)));
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "Healthy"
    }))
}
