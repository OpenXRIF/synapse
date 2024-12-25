use std::collections::HashMap;
use std::str::FromStr;

use serde_json;

#[derive(Debug, PartialEq)]
pub enum Modality {
    Text,
    ImageText,
}

#[derive(Debug, PartialEq)]
pub enum ModelProvider {
    Cohere, // Models from Cohere API
    Custom, // Models from Axon (ML platform)
    Local,  // Models loaded within the container level
}

impl FromStr for ModelProvider {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cohere" => Ok(ModelProvider::Cohere),
            "local" => Ok(ModelProvider::Local),
            "custom" => Ok(ModelProvider::Custom),
            _ => Err(()),
        }
    }
}

pub struct ModelConfig {
    pub name: String,
    pub provider: ModelProvider,
    pub modality: Modality,
    pub config_params: Option<serde_json::Value>,

    // Private params
    api_key: Option<String>,
}

pub trait ModelStrategy {
    fn new(config: ModelConfig) -> Self
    where
        Self: Sized;
    fn prompt(...) -> Any;
}

pub struct ModelConfig {
    pub provider: ModelProvider,
}
