pub struct Config {
    pub api_host: String,
    pub api_port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            api_host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            api_port: std::env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("PORT must be a number"),
        }
    }
}
