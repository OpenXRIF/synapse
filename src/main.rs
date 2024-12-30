mod api;
mod config;
mod constants;
mod database;
mod errors;
mod inference;
mod models;
mod prompting;
mod schema;
mod server;
mod services;
mod state;

use dotenvy::dotenv;

use crate::constants::WELCOME_ASCII;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("{}", WELCOME_ASCII);

    env_logger::init();
    dotenv().ok();

    let config: config::Config = config::Config::from_env();

    server::server(config).await
}
