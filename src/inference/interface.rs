use std::collections::HashMap;
use std::sync::Mutex;

use crate::inference::base::{ModelConfig, ModelProvider, ModelStrategy, ModelStrategyFactory};

pub struct ModelInterface {
    model_config: ModelConfig,
    strategy: Box<dyn ModelStrategy>,
}

impl ModelInterface {
    pub fn new(model_config: ModelConfig, factory: &dyn ModelStrategyFactory) -> Self {
        let strategy = factory.create_strategy(model_config.clone());
        ModelInterface {
            model_config,
            strategy,
        }
    }

    pub fn prompt(&self, prompt: String) -> String {
        self.strategy.prompt(prompt)
    }
}
