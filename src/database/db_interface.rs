use crate::config::Config;

pub struct DatabaseInterface;

impl DatabaseInterface {
    pub async fn new(config: &Config) -> Self {
        DatabaseInterface
    }

    pub async fn query(&self, query: &str) -> Option<String> {
        None
    }
}
