use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::Serialize;

#[derive(Debug, Display)]
pub enum ApiError {
    #[display("Internal Server Error")]
    InternalServerError,

    #[display("Not Found: {}", _0)]
    NotFound(String),

    #[display("Bad Request: {}", _0)]
    BadRequest(String),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::InternalServerError => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: self.to_string(),
                })
            }
            ApiError::NotFound(msg) => {
                HttpResponse::NotFound().json(ErrorResponse { error: msg.clone() })
            }
            ApiError::BadRequest(msg) => {
                HttpResponse::BadRequest().json(ErrorResponse { error: msg.clone() })
            }
        }
    }
}
