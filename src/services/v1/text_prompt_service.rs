use crate::errors::ApiError;
use crate::models::text_prompt_models::{TextPromptRequest, TextPromptResponse};

/// Process a text prompt
pub async fn process_prompt(request: TextPromptRequest) -> Result<TextPromptResponse, ApiError> {
    Ok(TextPromptResponse {
        response: format!("Prompt Format: {}", request.prompt_format),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn success() {
        let test_input = TextPromptRequest {
            prompt_format: "test_prompt_format".to_string(),
            prompt_args: Some(serde_json::json!(["example_prompt_arg"])),
            model_name: None,
            model_args: None,
        };

        let expected_result = TextPromptResponse {
            response: "Prompt Format: test_prompt_format".to_string(),
        };

        let result = process_prompt(test_input).await.unwrap();
        assert_eq!(result.response, expected_result.response);
    }
}
