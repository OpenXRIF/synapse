use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum PromptFormatArgType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "list")]
    List,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptFormat {
    pub prompt_format_name: String,
    pub prompt: String,
    pub prompt_args: HashMap<String, PromptFormatArgType>,
    pub rag_args: HashMap<String, String>,
    pub metadata: HashMap<String, String>,
}

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
