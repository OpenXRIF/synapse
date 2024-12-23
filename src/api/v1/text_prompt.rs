use crate::models::text_prompt_api::TextPromptRequest;
use crate::services::v1::text_prompt;
use actix_web::{web, HttpResponse};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/v1/text_prompt").route("", web::post().to(process_prompt)));
}

async fn process_prompt(
    request: web::Json<TextPromptRequest>,
) -> Result<HttpResponse, crate::errors::ApiError> {
    let response = text_prompt::process_prompt(request.into_inner()).await?;
    Ok(HttpResponse::Ok().json(response))
}
