use actix_web::{web, HttpResponse};
use log::info;

use crate::errors::ApiError;
use crate::models::text_prompt_models::TextPromptRequest;
use crate::services::v1::text_prompt_service;
use crate::state::AppState;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/waypoints")
            .route("", web::post().to(add_new_waypoints))
            .route("/{waypoint_set_name}", web::get().to(get_waypoints_by_set))
            .route("", web::put().to(update_waypoints))
            .route("/{waypoint_id}", web::delete().to(delete_waypoints))
    );
}

/// POST /v1/waypoints
async fn add_new_waypoints(
    request: web::Json<TextPromptRequest>,
    app_data: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    info!("POST /v1/waypoints: {:?}", request);
    Ok(HttpResponse::Ok().json())
}

/// GET /v1/waypoints/{waypoint_set_name}
async fn get_waypoints_by_set(app_data: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    info!("POST /v1/waypoints/: {:?}", request);
    Ok(HttpResponse::Ok().json())
}

/// PUT /v1/waypoints
async fn update_waypoints(
    request: web::Json<TextPromptRequest>,
    app_data: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    info!("POST /v1/waypoints: {:?}", request);
    Ok(HttpResponse::Ok().json())
}

/// DELETE /v1/waypoints/{waypoint_id}
async fn delete_waypoints(app_data: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    info!("POST /v1/waypoints/: {:?}", request);
    Ok(HttpResponse::Ok().json())
}
