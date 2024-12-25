use crate::inference::base::{Modality, ModelConfig, ModelInterface, ModelProvider};

pub struct CohereStrategy;

impl LLMStrategy for CohereStrategy {
    fn new(config: ModelConfig) -> Self {
        CohereStrategy
    }

    fn prompt(&self, prompt: String) -> String {
        prompt
    }
}
