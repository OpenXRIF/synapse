use std::cell::Cell;

use crate::errors::ApiError;
use crate::inference::{
    base::{Modality, ModelConfig, ModelProvider},
    interface::ModelInterface,
};
use crate::models::text_prompt_models::{TextPromptRequest, TextPromptResponse};

// Model Interface Singletons
thread_local!(
    pub static COHERE_INTERFACE: Cell<ModelInterface> = Cell::new(ModelInterface::new(
        ModelConfig::new("cohere".to_string(), ModelProvider::Cohere, Modality::Text, None, None)
    ))
);

/// Process a text prompt
pub async fn process_prompt(request: TextPromptRequest) -> Result<TextPromptResponse, ApiError> {
    let interface: ModelInterface = ModelInterface::new(ModelConfig::new(
        "cohere".to_string(),
        ModelProvider::Cohere,
        Modality::Text,
        None,
        None,
    ));
    let response: String = interface.text_prompt(request.prompt_format);
    Ok(TextPromptResponse { response })
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
