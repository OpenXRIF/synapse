use crate::api;
use crate::config;

use actix_web::{web::Data, App, HttpServer};

pub async fn server(config: config::Config) -> std::io::Result<()> {
    println!("-------- Starting Synapse Server --------");

    let api_config = config.clone();
    HttpServer::new(move || {
        let app_data = Data::new(api::state::AppState::new(config.clone()));

        App::new()
            .app_data(app_data)
            .route("/", actix_web::web::get().to(|| async { "Welcome!" }))
            .configure(api::health::configure)
            .configure(api::v1::text_prompt::configure)
    })
    .bind((api_config.api_host, api_config.api_port))?
    .run()
    .await
}
