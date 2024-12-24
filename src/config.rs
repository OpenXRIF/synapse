pub struct Config {
    pub api_host: String,
    pub api_port: u16,
    pub cohere_api_key: String,
}

impl Config {
    pub fn from_env() -> Self {
        let api_host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let api_port = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .expect("PORT must be a number");

        let cohere_api_key = match std::env::var("COHERE_API_KEY") {
            Ok(cohere_api_key) => cohere_api_key.to_string(),
            Err(_) => panic!("COHERE_API_KEY must be set"),
        };

        Self {
            api_host: api_host,
            api_port: api_port,
            cohere_api_key: cohere_api_key,
        }
    }
}
