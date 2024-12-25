use crate::inference::{
    base::ModelConfig,
    factory::{ModelStrategy, ModelStrategyFactory},
};

pub struct ModelInterface {
    model_config: ModelConfig,
    strategy: Box<dyn ModelStrategy>,
}

impl ModelInterface {
    pub fn new(model_config: ModelConfig) -> Self {
        let strategy: Box<dyn ModelStrategy> =
            ModelStrategyFactory::get_strategy(model_config.clone());
        ModelInterface {
            model_config,
            strategy,
        }
    }

    pub fn text_prompt(&self, prompt: String) -> String {
        self.strategy.text_prompt(prompt)
    }
}
