use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPromptFormatResponse {
    pub response: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePromptFormatRequest {
    pub prompt_format_name: String,
    pub prompt: String,
    pub prompt_args: Option<serde_json::Value>,
    pub rag_args: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,

    #[serde(default)]
    pub load_to_cache: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePromptFormatResponse {
    pub response: String,
}
