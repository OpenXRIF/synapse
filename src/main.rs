extern crate derive_more;
extern crate env_logger;
extern crate log;

use dotenvy::dotenv;

mod api;
mod config;
mod errors;
mod inference;
mod models;
mod server;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();

    let config: config::Config = config::Config::from_env();

    server::server(config).await
}
