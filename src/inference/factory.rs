use crate::inference::{
    base::{ModelConfig, ModelProvider},
    strategies::{
        cohere_interface::CohereStrategy, custom_interface::CustomStrategy,
        local_interface::LocalStrategy,
    },
};

pub trait ModelStrategy: Send + Sync {
    fn initialize(config: ModelConfig) -> Self
    where
        Self: Sized;

    fn text_prompt(&self, input: String) -> String;
}

pub struct ModelStrategyFactory;

impl ModelStrategyFactory {
    pub fn get_strategy(config: ModelConfig) -> Box<dyn ModelStrategy> {
        match config.provider {
            ModelProvider::Cohere => Box::new(CohereStrategy::initialize(config)),
            ModelProvider::Custom => Box::new(CustomStrategy::initialize(config)),
            ModelProvider::Local => Box::new(LocalStrategy::initialize(config)),
        }
    }
}
