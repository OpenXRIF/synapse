use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Modality {
    Text,
    ImageText,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
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

#[derive(Debug, Clone)]
pub struct ModelConfig {
    pub name: String,
    pub provider: ModelProvider,
    pub modality: Modality,
    pub api_key: Option<String>,
    pub config_params: Option<HashMap<String, String>>,
}

impl ModelConfig {
    pub fn new(
        name: String,
        provider: ModelProvider,
        modality: Modality,
        api_key: Option<String>,
        config_params: Option<HashMap<String, String>>,
    ) -> Self {
        ModelConfig {
            name,
            provider,
            modality,
            api_key,
            config_params,
        }
    }
}
