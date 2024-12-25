use crate::errors::ApiError;
use crate::inference::{
    base::{Modality, ModelConfig, ModelProvider},
    factory::ModelStrategyFactory,
    interface::ModelInterface,
};
use crate::models::text_prompt_models::{TextPromptRequest, TextPromptResponse};

// static COHERE_CONFIG: ModelConfig = ModelConfig::new("cohere".to_string(), ModelProvider::Cohere, Modality::Text, None, None);
// static COHERE_INTERFACE: ModelInterface = ModelInterface::new(COHERE_CONFIG);

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
        let test_input: TextPromptRequest = TextPromptRequest {
            prompt_format: "test_prompt_format".to_string(),
            prompt_args: Some(serde_json::json!(["example_prompt_arg"])),
            model_name: None,
            model_args: None,
        };

        let expected_result: TextPromptResponse = TextPromptResponse {
            response: "Prompt Format: test_prompt_format".to_string(),
        };

        let result: TextPromptResponse = process_prompt(test_input).await.unwrap();
        assert_eq!(result.response, expected_result.response);
    }
}
