use async_trait::async_trait;

use crate::config::Config;

#[async_trait]
pub trait DatabaseClient {
    async fn new(config: &Config) -> Self;
    async fn query(&self, query: &str) -> Option<String>;
}
