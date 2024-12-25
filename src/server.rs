use crate::api;
use crate::config;

use actix_web::{App, HttpServer};

pub async fn server(config: config::Config) -> std::io::Result<()> {
    println!("------ Starting Synapse Server ------");

    HttpServer::new(move || {
        App::new()
            .route("/", actix_web::web::get().to(|| async { "Welcome!" }))
            .configure(api::health::configure)
            .configure(api::v1::text_prompt::configure)
    })
    .bind((config.api_host, config.api_port))?
    .run()
    .await
}
