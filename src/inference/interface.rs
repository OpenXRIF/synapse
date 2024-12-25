use crate::inference::{
    base::ModelConfig,
    factory::{ModelStrategy, ModelStrategyFactory},
};

pub struct InterfaceMapper;
impl InterfaceMapper {
    pub fn get_interface(config: ModelConfig) -> ModelInterface {
        ModelInterface::new(config)
    }
}

pub struct ModelInterface {
    strategy: Box<dyn ModelStrategy>,
}

impl ModelInterface {
    pub fn new(model_config: ModelConfig) -> Self {
        let strategy: Box<dyn ModelStrategy> = ModelStrategyFactory::get_strategy(model_config);
        ModelInterface { strategy }
    }

    pub fn text_prompt(&self, prompt: String) -> String {
        self.strategy.text_prompt(prompt)
    }
}
