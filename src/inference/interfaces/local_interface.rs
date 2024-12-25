use crate::inference::base::{Modality, ModelConfig, ModelInterface, ModelProvider};

pub struct LocalStrategy;

impl LLMStrategy for LocalStrategy {
    fn new(_config: ModelConfig) -> Self {
        LocalStrategy
    }

    fn prompt(&self, prompt: String) -> String {
        prompt
    }
}
