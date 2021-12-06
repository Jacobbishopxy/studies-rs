use mongodb::{error::Error as MongoError, Client};

use crate::{
    errors::CustomError,
    model::{Planet, PlanetType},
};

const DB_NAME: &str = "solar_system_info";
const COLLECTION_NAME: &str = "planets";

pub struct MongoDbClient {
    client: Client,
}

impl MongoDbClient {
    // 连接 MongoDB
    pub async fn new(uri: &str) -> Result<Self, MongoError> {
        let client = Client::with_uri_str(uri).await?;

        Ok(MongoDbClient { client })
    }

    // R
    pub async fn get_planets(
        &self,
        planet_type: Option<PlanetType>,
    ) -> Result<Vec<Planet>, CustomError> {
        todo!()
    }
}
