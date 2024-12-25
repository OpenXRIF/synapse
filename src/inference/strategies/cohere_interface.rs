use crate::inference::{base::ModelConfig, factory::ModelStrategy};

pub struct CohereStrategy {
    client: String,
    _api_key: String,
}

impl ModelStrategy for CohereStrategy {
    fn new(config: ModelConfig) -> Self {
        CohereStrategy {
            client: config.name,
            _api_key: config.api_key.unwrap(),
        }
    }

    fn prompt(&self, prompt: String) -> String {
        prompt
    }
}
