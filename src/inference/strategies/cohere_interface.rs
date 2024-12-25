use crate::inference::{base::ModelConfig, factory::ModelStrategy};

pub struct CohereStrategy {
    client: String,
    _api_key: String,
}

impl ModelStrategy for CohereStrategy {
    fn initialize(config: ModelConfig) -> Self {
        match config.api_key {
            None => panic!("API key is required for Cohere models"),
            Some(_) => CohereStrategy {
                client: config.name,
                _api_key: config.api_key.unwrap(),
            },
        }
    }

    fn text_prompt(&self, prompt: String) -> String {
        self.client.clone() + &prompt
    }
}
