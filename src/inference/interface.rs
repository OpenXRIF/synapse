use std::collections::HashMap;
use std::sync::Mutex;

use crate::inference::base::{ModelConfig, ModelProvider, ModelStrategy, ModelStrategyFactory};

pub struct ModelInterface {
    model_config: ModelConfig,
    _strategy: Box<dyn ModelStrategy>,
}

impl ModelInterface {
    pub fn new(model_config: ModelConfig) -> Self {
        ModelInterface { model_config }
    }

    pub fn prompt(&self, prompt: String) -> String {
        let factory = ModelStrategyFactory::new();
        let strategy = factory.create_strategy(self.model_config.clone());
        strategy.prompt(prompt)
    }
}
