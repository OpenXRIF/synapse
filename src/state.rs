use std::collections::HashMap;
use std::sync::Arc;

use crate::config;
use crate::inference::{
    base::{Modality, ModelConfig, ModelProvider},
    interface::ModelInterface,
};

pub struct AppState {
    pub model_interfaces: Arc<HashMap<String, ModelInterface>>,
}

impl AppState {
    pub fn new(config: config::Config) -> Self {
        let mut interfaces: HashMap<String, ModelInterface> = HashMap::new();

        let cohere_config: ModelConfig = ModelConfig::new(
            "cohere".to_string(),
            ModelProvider::Cohere,
            Modality::Text,
            Some(config.cohere_api_key),
            None,
        );

        interfaces.insert("cohere".to_string(), ModelInterface::new(cohere_config));

        AppState {
            model_interfaces: Arc::new(interfaces),
        }
    }
}
