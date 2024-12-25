use crate::errors::ApiError;
use crate::inference::interface::ModelInterface;
use crate::models::text_prompt_models::{TextPromptRequest, TextPromptResponse};

/// Process a text prompt
pub async fn process_prompt(
    request: TextPromptRequest,
    interface: ModelInterface,
) -> Result<TextPromptResponse, ApiError> {
    // TODO: Build Prompt before sending to model
    let response: String = interface.text_prompt(request.prompt_format);
    Ok(TextPromptResponse { response })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inference::{
        base::{Modality, ModelConfig, ModelProvider},
        interface::ModelInterface,
    };

    #[tokio::test]
    async fn success() {
        let test_interface: ModelInterface = ModelInterface::new(ModelConfig::new(
            "cohere".to_string(),
            ModelProvider::Cohere,
            Modality::Text,
            None,
            Some("cohere_test_api_key".to_string()),
        ));

        let test_input: TextPromptRequest = TextPromptRequest {
            prompt_format: "test_prompt_format".to_string(),
            prompt_args: Some(serde_json::json!(["example_prompt_arg"])),
            model_name: None,
            model_args: None,
        };

        let _: TextPromptResponse = process_prompt(test_input, test_interface).await.unwrap();
    }
}
