use std::collections::HashMap;
use std::sync::Mutex;

use crate::inference::base::{ModelConfig, ModelProvider, ModelStrategy};
use crate::inference::interfaces::{
    cohere_interface::CohereStrategy, local_interface::LocalStrategy,
};

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
