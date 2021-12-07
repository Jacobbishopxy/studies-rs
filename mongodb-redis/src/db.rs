use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error as MongoError,
    Client,
};
use tokio_stream::StreamExt;

use crate::{
    errors::CustomError,
    model::{Planet, PlanetType},
};

const DB_NAME: &str = "solar_system_info";
const COLLECTION_NAME: &str = "planets";

#[derive(Clone)]
pub struct MongoDbClient {
    client: Client,
}

impl MongoDbClient {
    // 连接 MongoDB
    pub async fn new(uri: &str) -> Result<Self, MongoError> {
        let client = Client::with_uri_str(uri).await?;

        Ok(MongoDbClient { client })
    }

    // 根据类型读取所有数据
    pub async fn get_planets(
        &self,
        planet_type: Option<PlanetType>,
    ) -> Result<Vec<Planet>, CustomError> {
        // 根据类型设定筛选条件
        let filter = planet_type.map(|pt| {
            doc! {"type": pt.to_string()}
        });

        // 根据类型获取数据
        let mut planets = self.get_planets_collection().find(filter, None).await?;

        let mut result = Vec::new();
        // 由 tokio_stream 提供的 StreamExt 进行流式读取
        // StreamExt 介绍：
        // An extension trait for the [`Stream`] trait that provides a variety of
        // convenient combinator functions.
        while let Some(planet) = planets.next().await {
            result.push(planet?);
        }

        Ok(result)
    }

    // 根据ID读取单条数据
    pub async fn get_planet(&self, id: ObjectId) -> Result<Planet, CustomError> {
        let collection = self.get_planets_collection();

        let filter = doc! {"_id": &id};
        collection
            .find_one(filter, None)
            .await?
            .ok_or(CustomError::NotFound {
                message: format!("Can't find a planet by id: {}", &id),
            })
    }

    // 获取数据库下的 collection 对象
    fn get_planets_collection(&self) -> mongodb::Collection<Planet> {
        self.client.database(DB_NAME).collection(COLLECTION_NAME)
    }
}
