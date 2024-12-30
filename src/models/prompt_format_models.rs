use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum PromptFormatArgType {
    #[serde(rename = "string")]
    String(String),
    #[serde(rename = "int")]
    Int(i32),
    #[serde(rename = "bool")]
    Bool(bool),
    #[serde(rename = "list")]
    List(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptFormat {
    pub format_name: String,
    pub prompt: String,
    pub prompt_args: HashMap<String, PromptFormatArgType>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPromptFormatResponse {
    pub response: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePromptFormatRequest {
    pub name: String,
    pub prompt: String,
    pub prompt_args: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,

    #[serde(default)]
    pub load_to_cache: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePromptFormatResponse {
    pub response: String,
}
