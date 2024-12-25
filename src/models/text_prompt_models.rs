use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TextPromptRequest {
    pub prompt_format: String,
    pub prompt_args: Option<serde_json::Value>,
    pub model_name: Option<String>,
    pub model_args: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextPromptResponse {
    pub response: String,
}
