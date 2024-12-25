use log::info;

use crate::errors::ApiError;
use crate::models::text_prompt_models::{TextPromptRequest, TextPromptResponse};

/// Process a text prompt
pub async fn process_prompt(request: TextPromptRequest) -> Result<TextPromptResponse, ApiError> {
    info!("Processing prompt: {}", request.prompt_format);
    Ok(TextPromptResponse {
        response: format!("Prompt Format: {}", request.prompt_format),
    })
}
