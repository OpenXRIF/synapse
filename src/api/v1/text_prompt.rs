use crate::errors::ApiError;
use crate::models::text_prompt_models::{TextPromptRequest, TextPromptResponse};
use crate::services::v1::text_prompt_service;
use actix_web::{web, HttpResponse};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/v1/text_prompt").route("", web::post().to(process_prompt)));
}

async fn process_prompt(request: web::Json<TextPromptRequest>) -> Result<HttpResponse, ApiError> {
    // POST /v1/text_prompt
    let response: TextPromptResponse =
        text_prompt_service::process_prompt(request.into_inner()).await?;
    Ok(HttpResponse::Ok().json(response))
}
