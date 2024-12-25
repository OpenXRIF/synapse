use crate::inference::base::{Modality, ModelConfig, ModelInterface, ModelProvider};

pub struct CohereStrategy;

impl LLMStrategy for CohereStrategy {
    fn new(_config: ModelConfig) -> Self {
        CohereStrategy
    }

    fn prompt(&self, prompt: String) -> String {
        prompt
    }
}
