use crate::inference::base::ModelConfig;

use rig::completion::Prompt;
use rig::providers::cohere::Client as CohereClient;

// TODO: Modify this struct to implement ModelStrategy

pub struct ModelInterface {
    cohere_client: CohereClient,
}

impl ModelInterface {
    pub fn new(model_config: ModelConfig) -> Self {
        match model_config.api_key {
            None => panic!("API key is required for Cohere models"),
            Some(api_key) => ModelInterface {
                cohere_client: CohereClient::new(&api_key),
            },
        }
    }

    pub async fn text_prompt(&self, input: String) -> String {
        let model = self
            .cohere_client
            .agent("command-r-plus-08-2024")
            .temperature(0.5)
            .build();

        model
            .prompt(&input)
            .await
            .expect("Failed to get response from Cohere")
    }
}

/* TODO: Implement ModelStrategy for ModelInterface
use crate::inference::{
    base::ModelConfig,
    factory::{ModelStrategy, ModelStrategyFactory},
};

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
*/
