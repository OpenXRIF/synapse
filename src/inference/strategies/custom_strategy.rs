use crate::inference::{base::ModelConfig, factory::ModelStrategy};

pub struct CustomStrategy;

impl ModelStrategy for CustomStrategy {
    fn initialize(_config: ModelConfig) -> Self {
        CustomStrategy
    }

    fn text_prompt(&self, prompt: String) -> String {
        prompt
    }
}
