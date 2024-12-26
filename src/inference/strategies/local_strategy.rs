use crate::inference::{base::ModelConfig, factory::ModelStrategy};

pub struct LocalStrategy;

impl ModelStrategy for LocalStrategy {
    fn initialize(_config: ModelConfig) -> Self {
        LocalStrategy
    }

    fn text_prompt(&self, prompt: String) -> String {
        prompt
    }
}
