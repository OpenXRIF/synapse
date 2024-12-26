use std::collections::HashMap;
use tokio::sync::Mutex;

use crate::api;
use crate::config;
use crate::inference::{
    base::{Modality, ModelConfig, ModelProvider},
    interface::ModelInterface,
};

use actix_web::{web::Data, App, HttpServer};

struct AppState {
    model_interfaces: Mutex<HashMap<String, ModelInterface>>,
}

impl AppState {
    pub fn new(config: config::Config) -> Self {
        let mut interfaces: HashMap<String, ModelInterface> = HashMap::new();

        let cohere_config = ModelConfig::new(
            "cohere".to_string(),
            ModelProvider::Cohere,
            Modality::Text,
            None,
            Some(config.cohere_api_key),
        );

        interfaces.insert("cohere".to_string(), ModelInterface::new(cohere_config));

        AppState {
            model_interfaces: Mutex::new(interfaces),
        }
    }
}

pub async fn server(config: config::Config) -> std::io::Result<()> {
    println!("-------- Starting Synapse Server --------");

    let api_config = config.clone();
    HttpServer::new(move || {
        let app_data = Data::new(AppState::new(config.clone()));

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
