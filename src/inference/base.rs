use std::any::Any;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Mutex;

use crate::inference::interfaces::{
    cohere_interface::CohereStrategy, local_interface::LocalStrategy,
};

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

#[derive(Debug, Clone)]
pub struct ModelConfig {
    pub name: String,
    pub provider: ModelProvider,
    pub modality: Modality,
    pub config_params: Option<HashMap<String, Any>>,

    // Private params
    api_key: Option<String>,
}

pub trait ModelStrategy {
    fn new(config: ModelConfig) -> Self
    where
        Self: Sized;

    fn prompt(...) -> Any;
}

pub struct ModelStrategyFactory {
    strategies: Mutex<HashMap<ModelProvider, Arc<dyn Fn(ModelConfig) -> Box<dyn ModelStrategy>>>>,
}

impl ModelStrategyFactory {
    pub fn new() -> Self {
        let mut strategies: HashMap<
            ModelProvider,
            Arc<dyn Fn(ModelConfig) -> Box<dyn ModelStrategy>>,
        > = HashMap::new();

        strategies.insert(
            ModelProvider::Cohere,
            Arc::new(|config| Box::new(CohereStrategy::new(config))),
        );
        strategies.insert(
            ModelProvider::Local,
            Arc::new(|config| Box::new(LocalStrategy::new(config))),
        );

        ModelStrategyFactory {
            strategies: Mutex::new(strategies),
        }
    }

    pub fn register_strategy<F>(&self, provider: ModelProvider, strategy_fn: F)
    where
        F: Fn(ModelConfig) -> Box<dyn ModelStrategy> + 'static,
    {
        self.strategies
            .lock()
            .unwrap()
            .insert(provider, Box::new(strategy_fn));
    }

    pub fn create_strategy(&self, config: ModelConfig) -> Box<dyn ModelStrategy> {
        let strategies = self.strategies.lock().unwrap();
        let strategy_fn = strategies
            .get(&config.provider)
            .expect("Unsupported model provider");
        strategy_fn(config)
    }
}
