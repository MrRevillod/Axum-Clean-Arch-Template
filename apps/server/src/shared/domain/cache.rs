use async_trait::async_trait;
use serde_json::Value;
use shaku::Interface;

#[derive(Debug)]
pub struct CacheError(pub String);

#[async_trait]
pub trait Cache: Interface {
    async fn get_str(&self, key: &str) -> Result<Option<String>, CacheError>;
    async fn set_str(&self, key: &str, value: String) -> Result<(), CacheError>;

    async fn get_json(&self, key: &str) -> Result<Option<Value>, CacheError>;
    async fn set_json(&self, key: &str, value: &Value) -> Result<(), CacheError>;
}
