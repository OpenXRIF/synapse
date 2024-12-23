use actix_web::{App, HttpServer};
use dotenvy::dotenv;

mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = config::Config::from_env();

    HttpServer::new(move || {
        App::new().route("/", actix_web::web::get().to(|| async { "Hello, Actix!" }))
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
