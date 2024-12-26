use rig::{completion::Prompt, providers::cohere::Client as CohereClient};

use crate::inference::{base::ModelConfig, factory::ModelStrategy};

pub struct CohereStrategy {
    client: CohereClient,
}

impl ModelStrategy for CohereStrategy {
    fn initialize(config: ModelConfig) -> Self {
        match config.api_key {
            None => panic!("API key is required for Cohere models"),
            Some(api_key) => CohereStrategy {
                client: CohereClient::new(&api_key),
            },
        }
    }

    #[tokio::main]
    async fn text_prompt(&self, prompt: String) -> String {
        let model = self
            .client
            .agent("command-r-plus-08-2024")
            .temperature(0.5)
            .build();

        model
            .prompt(&prompt)
            .await
            .expect("Failed to get response from Cohere")
    }
}
