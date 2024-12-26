mod api;
mod config;
mod constants;
mod errors;
mod inference;
mod models;
mod server;
mod services;

use crate::constants::WELCOME_ASCII;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("{}", WELCOME_ASCII);

    env_logger::init();
    dotenv().ok();

    let config: config::Config = config::Config::from_env();

    server::server(config).await
}

// use rig::{
//     completion::Prompt,
//     providers::cohere::Client as CohereClient,
// };

// #[tokio::main]
// async fn main() {
//     let client = CohereClient::new(std::env::var("COHERE_API_KEY").expect("COHERE_API_KEY not set").as_str());
//     let response = client.agent("command-r-plus-08-2024")
//         .temperature(0.5)
//         .build()
//         .prompt("What is the capital of the United States?")
//         .await
//         .expect("Failed to get response from Cohere");

//     println!("{}", response);
// }
