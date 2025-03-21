#[derive(Clone)]
pub struct Config {
    pub api_host: String,
    pub api_port: u16,
    pub database_type: String,
    pub database_url: String,
    pub cohere_api_key: String,
}

impl Config {
    pub fn from_env() -> Self {
        // API Configuration
        let api_host: String = std::env::var("HOST").unwrap_or_else(|_| "localhost".to_string());
        let api_port: u16 = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .expect("PORT must be a number");

        // Database Configuration
        let db_type: String =
            std::env::var("DATABASE_TYPE").unwrap_or_else(|_| "sqlite".to_string());
        let db_url: String =
            std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://test.db".to_string());

        // Model Provider API Keys
        let cohere_api_key: String =
            std::env::var("COHERE_API_KEY").expect("COHERE_API_KEY not set");

        Self {
            api_host: api_host,
            api_port: api_port,
            database_type: db_type,
            database_url: db_url,
            cohere_api_key: cohere_api_key,
        }
    }

    pub fn test() -> Self {
        Self {
            api_host: "localhost".to_string(),
            api_port: 8080,
            database_type: "test_db_type".to_string(),
            database_url: "test_db_url".to_string(),
            cohere_api_key: "test_cohere_key".to_string(),
        }
    }
}
