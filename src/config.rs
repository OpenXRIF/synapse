pub struct Config {
    pub api_host: String,
    pub api_port: u16,
    pub cohere_api_key: String,
}

impl Config {
    pub fn from_env() -> Self {
        let api_host: String = std::env::var("HOST").unwrap_or_else(|_| "localhost".to_string());
        let api_port: u16 = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .expect("PORT must be a number");

        let cohere_api_key: String = match std::env::var("COHERE_API_KEY") {
            Ok(cohere_api_key) => cohere_api_key.to_string(),
            Err(_) => panic!("COHERE_API_KEY must be set"),
        };

        Self {
            api_host: api_host,
            api_port: api_port,
            cohere_api_key: cohere_api_key,
        }
    }

    pub fn test() -> Self {
        Self {
            api_host: "localhost".to_string(),
            api_port: 8080,
            cohere_api_key: "test_cohere_key".to_string(),
        }
    }
}
