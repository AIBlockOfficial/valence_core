use async_trait::async_trait;
use mongodb::bson::{doc, DateTime, Document};
use mongodb::{options::ClientOptions, Client};
use serde::{de::DeserializeOwned, Serialize};
use tracing::{event, span, trace, warn, Level};

use super::handler::KvStoreConnection;

#[derive(Debug, Clone)]
pub struct MongoDbIndex {
    pub db_name: String,
    pub coll_name: String,
}

#[derive(Debug, Clone)]
pub struct MongoDbConn {
    pub client: Client,
    pub index: MongoDbIndex,
}

impl MongoDbConn {
    /// Creates a TTL index on the expiry field
    pub async fn create_ttl_index(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let collection = self
            .client
            .database(&self.index.db_name)
            .collection::<Document>(&self.index.coll_name);

        // Create TTL index on the 'expiry' field
        let index_model = mongodb::IndexModel::builder()
            .keys(doc! { "expiry": 1 })
            .options(Some(
                mongodb::options::IndexOptions::builder()
                    .expire_after(Some(std::time::Duration::from_secs(0)))
                    .build(),
            ))
            .build();

        collection.create_index(index_model, None).await?;
        Ok(())
    }
}

#[async_trait]
impl KvStoreConnection for MongoDbConn {
    async fn init(url: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Tracing
        let span = span!(Level::TRACE, "MongoDbConn::init");
        let _enter = span.enter();

        let client_options = match ClientOptions::parse(url).await {
            Ok(client_options) => client_options,
            Err(e) => panic!("Failed to connect to MongoDB instance with error: {e}"),
        };

        trace!("Connected to MongoDB instance at {url}");

        let client = match Client::with_options(client_options) {
            Ok(client) => client,
            Err(e) => panic!("Failed to connect to MongoDB instance with error: {e}"),
        };

        trace!("MongoDB client created successfully");

        let index = MongoDbIndex {
            db_name: String::from("default"),
            coll_name: String::from("default"),
        };

        Ok(MongoDbConn { client, index })
    }

    async fn set_data<T: Serialize + std::marker::Send>(
        &mut self,
        key: &str,
        value: T,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Tracing
        let span = span!(Level::TRACE, "MongoDbConn::set_data");
        let _enter = span.enter();

        let collection = self
            .client
            .database(&self.index.db_name)
            .collection::<Document>(&self.index.coll_name);

        let document = match mongodb::bson::to_document(&value) {
            Ok(document) => document,
            Err(e) => {
                event!(Level::ERROR, "Failed to serialize data with error: {e}");
                Document::new()
            }
        };

        let filter = doc! { "_id": key };
        match collection
            .replace_one(
                filter,
                document.clone(),
                mongodb::options::ReplaceOptions::builder()
                    .upsert(true)
                    .build(),
            )
            .await
        {
            Ok(_) => (),
            Err(e) => {
                event!(Level::ERROR, "Failed to set data with error: {e}");
            }
        };

        trace!("Data set successfully");

        Ok(())
    }

    async fn set_data_with_expiry<T: Serialize + std::marker::Send>(
        &mut self,
        key: &str,
        value: T,
        seconds: usize,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Tracing
        let span = span!(Level::TRACE, "MongoDbConn::set_data_with_expiry");
        let _enter = span.enter();

        let collection = self
            .client
            .database(&self.index.db_name)
            .collection::<Document>(&self.index.coll_name);

        let mut document = match mongodb::bson::to_document(&value) {
            Ok(document) => document,
            Err(e) => {
                event!(Level::ERROR, "Failed to serialize data with error: {e}");
                Document::new()
            }
        };

        let milli_expiry = (seconds * 1000) as i64;
        let expiry_time = DateTime::from_millis(milli_expiry);
        document.insert("_id", key);
        document.insert("expiry", expiry_time);

        // Set the data with upsert option
        let filter = doc! { "_id": key };
        match collection
            .replace_one(
                filter,
                document.clone(),
                mongodb::options::ReplaceOptions::builder()
                    .upsert(true)
                    .build(),
            )
            .await
        {
            Ok(_) => (),
            Err(e) => {
                event!(Level::ERROR, "Failed to set data with error: {e}");
            }
        };

        trace!("Data set successfully with expiry");

        Ok(())
    }

    async fn delete_data(
        &mut self,
        key: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Tracing
        let span = span!(Level::TRACE, "MongoDbConn::delete_data");
        let _enter = span.enter();

        let collection = self
            .client
            .database(&self.index.db_name)
            .collection::<Document>(&self.index.coll_name);

        let filter = doc! { "_id": key };
        match collection.delete_one(filter, None).await {
            Ok(_) => (),
            Err(e) => {
                event!(Level::ERROR, "Failed to delete data with error: {e}");
            }
        };

        trace!("Data deleted successfully");

        Ok(())
    }

    async fn get_data<T: DeserializeOwned>(
        &mut self,
        key: &str,
    ) -> Result<Option<T>, Box<dyn std::error::Error + Send + Sync>> {
        // Tracing
        let span = span!(Level::TRACE, "MongoDbConn::get_data");
        let _enter = span.enter();

        let collection = self
            .client
            .database(&self.index.db_name)
            .collection::<Document>(&self.index.coll_name); // Change to your actual collection name

        let filter = doc! { "_id": key };
        let result = collection.find_one(filter, None).await?;

        trace!("Data retrieved successfully");

        if let Some(document) = result {
            let deserialized: T = mongodb::bson::from_document(document)?;
            return Ok(Some(deserialized));
        }

        warn!("Data unsuccessfully deserialized");

        Ok(None)
    }
}
