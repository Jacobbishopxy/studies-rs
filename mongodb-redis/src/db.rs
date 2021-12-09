use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    error::Error as MongoError,
    Client,
};
use rust_embed::RustEmbed;
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

    // 创建一个新的数据
    pub async fn create_planet(&self, planet: Planet) -> Result<Planet, CustomError> {
        let collection = self.get_planets_collection();

        let insert_result = collection.insert_one(planet, None).await?;
        let filter = doc! {"_id": &insert_result.inserted_id};
        collection
            .find_one(filter, None)
            .await?
            .ok_or(CustomError::NotFound {
                message: "Planet not found".to_string(),
            })
    }

    // 根据 ID 读取单条数据
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

    // 根据 ID 更新单条数据
    pub async fn update_planet(&self, id: ObjectId, planet: Planet) -> Result<Planet, CustomError> {
        let collection = self.get_planets_collection();

        let query = doc! {"_id": &id};
        let update = doc! {
            "$set": Document::from(&planet)
        };
        let _update_result = collection.update_one(query.clone(), update, None).await?;

        collection
            .find_one(query, None)
            .await?
            .ok_or(CustomError::NotFound {
                message: format!("Can't find a planet by id: {}", &id),
            })
    }

    // 根据 ID 删除单条数据
    pub async fn delete_planet(&self, id: ObjectId) -> Result<(), CustomError> {
        let collection = self.get_planets_collection();

        let filter = doc! {"_id": &id};
        let _delete_result =
            collection
                .find_one_and_delete(filter, None)
                .await?
                .ok_or(CustomError::NotFound {
                    message: format!("Can't find a planet by id: {}", &id),
                })?;

        Ok(())
    }

    // 获取数据库下的 collection 对象
    fn get_planets_collection(&self) -> mongodb::Collection<Planet> {
        self.client.database(DB_NAME).collection(COLLECTION_NAME)
    }
}

#[derive(RustEmbed)]
#[folder = "images"]
struct Asset;

pub async fn get_image_of_planet(planet_name: &str) -> Vec<u8> {
    let filename = format!("{}.jpg", planet_name.to_lowercase());
    let image = Asset::get(&filename).expect("Failed to open image");
    image.data.to_vec()
}
