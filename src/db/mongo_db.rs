use async_trait::async_trait;
use mongodb::bson::{doc, Document};
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

#[async_trait]
impl KvStoreConnection for MongoDbConn {
    async fn init(url: &str) -> Result<Self, Box<dyn std::error::Error>> {
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
    ) -> Result<(), Box<dyn std::error::Error>> {
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
                panic!("Failed to serialize data with error: {e}");
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
                panic!("Failed to set data with error: {e}");
            }
        };

        trace!("Data set successfully");

        Ok(())
    }

    async fn get_data<T: DeserializeOwned>(
        &mut self,
        key: &str,
    ) -> Result<Option<T>, Box<dyn std::error::Error>> {
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
