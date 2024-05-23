use crate::db::handler::{CacheHandler, KvStoreConnection};
use async_trait::async_trait;
use redis::{aio::ConnectionManager, AsyncCommands};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Clone)]
pub struct RedisCacheConn {
    pub connection: ConnectionManager,
}

#[async_trait]
impl CacheHandler for RedisCacheConn {
    async fn expire_entry(
        &mut self,
        key: &str,
        seconds: usize,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.connection.expire(key, seconds).await?;
        Ok(())
    }
}

#[async_trait]
impl KvStoreConnection for RedisCacheConn {
    async fn init(url: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let redis_client = redis::Client::open(url)?;
        let redis_connection_manager = ConnectionManager::new(redis_client).await?;

        Ok(RedisCacheConn {
            connection: redis_connection_manager,
        })
    }

    async fn set_data<T: Serialize + DeserializeOwned + Send>(
        &mut self,
        key: &str,
        value: T,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let exists: bool = self.connection.exists(key).await?;

        let mut vec: Vec<T> = if exists {
            // Get the existing data
            let data: String = self.connection.get(key).await?;
            serde_json::from_str(&data)?
        } else {
            Vec::new()
        };

        // Append the new data to the vec
        vec.push(value);

        let serialized = serde_json::to_string(&vec)?;
        self.connection.set(key, serialized).await?;

        Ok(())
    }

    async fn set_data_with_expiry<T: Serialize + DeserializeOwned + Send>(
        &mut self,
        key: &str,
        value: T,
        seconds: usize,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Check if the key exists
        let exists: bool = self.connection.exists(key).await?;

        let mut vec: Vec<T> = if exists {
            // Get the existing data
            let data: String = self.connection.get(key).await?;
            serde_json::from_str(&data)?
        } else {
            Vec::new()
        };

        // Append the new data to the vec
        vec.push(value);

        // Serialize the vec back to a string
        let serialized = serde_json::to_string(&vec)?;

        // Set the data back to Redis
        self.connection.set(key, serialized).await?;

        // Set the expiry time for the key
        self.connection.expire(key, seconds).await?;

        Ok(())
    }

    async fn delete_data(
        &mut self,
        key: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let _: () = self.connection.del(key).await?;
        Ok(())
    }

    async fn get_data<T: DeserializeOwned>(
        &mut self,
        key: &str,
    ) -> Result<Option<Vec<T>>, Box<dyn std::error::Error + Send + Sync>> {
        // Check if the key exists
        let exists: bool = self.connection.exists(key).await?;

        if exists {
            // Get the existing data
            let data: String = self.connection.get(key).await?;
            let vec: Vec<T> = serde_json::from_str(&data)?;
            return Ok(Some(vec));
        }

        Ok(None)
    }
}
