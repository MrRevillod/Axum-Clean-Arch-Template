use std::str::FromStr;

use async_trait::async_trait;
use redis::{aio::MultiplexedConnection, AsyncCommands, Client};
use serde_json::Value;
use shaku::Component;

use crate::shared::{
    constants::REDIS_CACHE_DB_URL,
    domain::cache::{Cache, CacheError},
};

#[derive(Component)]
#[shaku(interface = Cache)]
pub struct RedisCache {
    pub connection: MultiplexedConnection,
}

impl RedisCache {
    pub async fn new() -> Result<Self, CacheError> {
        let client = Client::open(REDIS_CACHE_DB_URL.as_str())
            .map_err(|e| CacheError(e.to_string()))?;

        let connection = client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| CacheError(e.to_string()))?;

        Ok(Self { connection })
    }

    pub fn get_connection(&self) -> MultiplexedConnection {
        self.connection.clone()
    }
}

#[async_trait]
impl Cache for RedisCache {
    async fn get_str(&self, key: &str) -> Result<Option<String>, CacheError> {
        let mut connection = self.get_connection();

        let cached: Option<String> = connection
            .get(key)
            .await
            .map_err(|e| CacheError(e.to_string()))?;

        let Some(s) = cached else {
            return Ok(None);
        };

        Ok(Some(s))
    }

    async fn set_str(&self, key: &str, value: String) -> Result<(), CacheError> {
        let mut connection = self.get_connection();

        connection
            .set::<&str, String, ()>(key, value)
            .await
            .map_err(|e| CacheError(format!("Redis set error: {}", e)))?;

        Ok(())
    }

    async fn get_json(&self, key: &str) -> Result<Option<Value>, CacheError> {
        let mut connection = self.get_connection();

        let cached: Option<String> = connection
            .get(key)
            .await
            .map_err(|e| CacheError(e.to_string()))?;

        let Some(s) = cached else {
            return Ok(None);
        };

        let Ok(value) = Value::from_str(&s) else {
            return Err(CacheError("JSON parse error".to_string()));
        };

        Ok(Some(value))
    }

    async fn set_json(&self, key: &str, value: &Value) -> Result<(), CacheError> {
        let mut connection = self.get_connection();

        let value_str = serde_json::to_string(value)
            .map_err(|e| CacheError(format!("JSON serialization error: {}", e)))?;

        connection
            .set::<&str, String, ()>(key, value_str)
            .await
            .map_err(|e| CacheError(format!("Redis set error: {}", e)))?;

        Ok(())
    }
}

impl Into<RedisCacheParameters> for RedisCache {
    fn into(self) -> RedisCacheParameters {
        RedisCacheParameters {
            connection: self.connection,
        }
    }
}
