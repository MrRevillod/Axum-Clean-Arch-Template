use async_trait::async_trait;
use redis::{aio::MultiplexedConnection, AsyncCommands, Client};
use shaku::Component;

use crate::shared::{
    constants::REDIS_CACHE_DB_URL,
    domain::{Cache, CacheError, CacheResult},
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
    async fn get(&self, key: &str) -> Result<CacheResult<String>, CacheError> {
        let mut connection = self.get_connection();

        let cached_value: Option<String> = connection
            .get(key)
            .await
            .map_err(|e| CacheError(e.to_string()))?;

        let Some(value) = cached_value else {
            return Ok(CacheResult::empty());
        };

        let etag = self.etag(&value);

        Ok(CacheResult::new(value, Some(etag)))
    }

    async fn set(&self, key: &str, value: String) -> Result<(), CacheError> {
        let mut connection = self.get_connection();

        connection
            .set::<&str, String, ()>(key, value)
            .await
            .map_err(|e| CacheError(format!("Redis set error: {}", e)))?;

        Ok(())
    }

    fn etag(&self, data: &str) -> String {
        format!("{:x}", md5::compute(data))
    }
}

impl Into<RedisCacheParameters> for RedisCache {
    fn into(self) -> RedisCacheParameters {
        RedisCacheParameters {
            connection: self.connection,
        }
    }
}
