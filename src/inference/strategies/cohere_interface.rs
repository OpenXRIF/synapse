use std::collections::HashMap;

use async_trait::async_trait;
use rig::{
    agent::Agent,
    providers::cohere::{Client as CohereClient, CompletionModel},
};

use crate::inference::{base::ModelConfig, factory::ModelStrategy};

pub struct CohereStrategy {
    client: CohereClient,
    models: HashMap<String, Agent<CompletionModel>>,
}

#[async_trait]
impl ModelStrategy for CohereStrategy {
    fn initialize(config: ModelConfig) -> Self {
        match config.api_key {
            None => panic!("API key is required for Cohere models"),
            Some(api_key) => {
                let client = CohereClient::new(&api_key);
                let mut models = HashMap::new();
                models.insert(
                    "default".to_string(),
                    client
                        .agent("command-r-plus-08-2024")
                        .temperature(0.5)
                        .build(),
                );
                CohereStrategy {
                    client: client,
                    models: models,
                }
            }
        }
    }

    async fn text_prompt<'a>(&'a self, input: String) -> String {
        let model = self
            .client
            .agent("command-r-plus-08-2024")
            .temperature(0.5)
            .build();

        model.prompt(&input).await
    }
}
