use std::collections::HashMap;

use crate::errors::ApiError;
use crate::inference::interface::ModelInterface;
use crate::models::text_prompt_models::{TextPromptRequest, TextPromptResponse};

/// Process a text prompt
pub async fn process_prompt(
    request: TextPromptRequest,
    interfaces: &HashMap<String, ModelInterface>,
) -> Result<TextPromptResponse, ApiError> {
    // TODO: Use Model Interface to get response
    let _interface: &ModelInterface;

    match request.model_name {
        Some(name) => match interfaces.get(&name) {
            Some(interface) => {
                _interface = interface;
            }
            None => return Err(ApiError::NotFound("Model not found".to_string())),
        },
        None => {
            _interface = interfaces.get("cohere").unwrap();
        }
    }

    // TODO: Build Prompt before sending to model

    Ok(TextPromptResponse {
        response: _interface.text_prompt(request.prompt_format).await,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inference::{
        base::{Modality, ModelConfig, ModelProvider},
        interface::ModelInterface,
    };

    // TODO: Modify this test after strategy pattern with Async implemented.
    // #[tokio::test]
    // async fn test_process_prompt_success() {
    //     let test_interface: ModelInterface = ModelInterface::new(ModelConfig::new(
    //         "cohere".to_string(),
    //         ModelProvider::Cohere,
    //         Modality::Text,
    //         Some("cohere_test_api_key".to_string()),
    //         None,
    //     ));

    //     let mut test_interfaces: HashMap<String, ModelInterface> = HashMap::new();
    //     test_interfaces.insert("cohere".to_string(), test_interface);

    //     let test_input: TextPromptRequest = TextPromptRequest {
    //         prompt_format: "test_prompt_format".to_string(),
    //         prompt_args: Some(serde_json::json!(["example_prompt_arg"])),
    //         model_name: None,
    //         model_args: None,
    //     };

    //     let _: TextPromptResponse = process_prompt(test_input, &test_interfaces).await.unwrap();
    // }

    #[tokio::test]
    async fn test_process_prompt_non_existent_model() {
        let test_interface: ModelInterface = ModelInterface::new(ModelConfig::new(
            "cohere".to_string(),
            ModelProvider::Cohere,
            Modality::Text,
            Some("cohere_test_api_key".to_string()),
            None,
        ));

        let mut test_interfaces: HashMap<String, ModelInterface> = HashMap::new();
        test_interfaces.insert("cohere".to_string(), test_interface);

        let test_input: TextPromptRequest = TextPromptRequest {
            prompt_format: "test_prompt_format".to_string(),
            prompt_args: Some(serde_json::json!(["example_prompt_arg"])),
            model_name: Some("non_existent_model".to_string()),
            model_args: None,
        };

        let response: Result<TextPromptResponse, ApiError> =
            process_prompt(test_input, &test_interfaces).await;
        assert_eq!(
            response.unwrap_err(),
            ApiError::NotFound("Model not found".to_string())
        );
    }
}
