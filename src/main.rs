use dotenvy::dotenv;

mod api;
mod config;
mod errors;
mod models;
mod server;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = config::Config::from_env();

    server::server(config).await
}
