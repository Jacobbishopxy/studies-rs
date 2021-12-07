//!
//!

use std::str::FromStr;

use log::debug;
use mongodb::bson::oid::ObjectId;
use redis::{aio::ConnectionManager, AsyncCommands, Client as RedisClient, Value};

use crate::{
    db::MongoDbClient,
    errors::CustomError,
    model::{Planet, PlanetType},
};

const PLANET_KEY_PREFIX: &str = "planet";
const IMAGE_KEY_PREFIX: &str = "image";
const RATE_LIMIT_KEY_PREFIX: &str = "rate_limit";
const MAX_REQUESTS_PER_MINUTE: u64 = 10;
pub const NEW_PLANETS_CHANNEL_NAME: &str = "new_planets";

#[derive(Clone)]
pub struct PlanetService {
    mongodb_client: MongoDbClient,
    redis_client: RedisClient,
    redis_connection_manager: ConnectionManager,
}

impl PlanetService {
    pub fn new(
        mongodb_client: MongoDbClient,
        redis_client: RedisClient,
        redis_connection_manager: ConnectionManager,
    ) -> Self {
        Self {
            mongodb_client,
            redis_client,
            redis_connection_manager,
        }
    }

    pub async fn get_planets(
        &self,
        planet_type: Option<PlanetType>,
    ) -> Result<Vec<Planet>, CustomError> {
        self.mongodb_client.get_planets(planet_type).await
    }

    pub async fn get_planet(&self, planet_id: &str) -> Result<Planet, CustomError> {
        let cache_key = self.get_planet_cache_key(planet_id);
        // 建立 Redis 异步连接
        let mut con = self.redis_client.get_async_connection().await?;

        // 从 Redis 中获取缓存的 planet_cache_key
        let cached_planet = con.get(&cache_key).await?;
        match cached_planet {
            // 如果不存在缓存，则从 MongoDB 中获取
            Value::Nil => {
                debug!("Use database to retrieve a planet by id: {}", &planet_id);
                let result: Planet = self
                    .mongodb_client
                    .get_planet(ObjectId::from_str(planet_id)?)
                    .await?;

                // Redis 原子操作，将 Planet 存入 Redis，并设置过期时间
                let _: () = redis::pipe()
                    .atomic()
                    .set(&cache_key, &result)
                    .expire(&cache_key, 60)
                    .query_async(&mut con)
                    .await?;

                Ok(result)
            }
            Value::Data(val) => {
                debug!("Use cache to retrieve a planet by id: {}", planet_id);
                Ok(serde_json::from_slice(&val)?)
            }
            _ => Err(CustomError::RedisError {
                message: "Unexpected response from Redis".to_string(),
            }),
        }
    }

    fn get_planet_cache_key(&self, planet_id: &str) -> String {
        format!("{}:{}", PLANET_KEY_PREFIX, planet_id)
    }

    fn get_image_cache_key(&self, planet_id: &str) -> String {
        format!("{}:{}:{}", PLANET_KEY_PREFIX, planet_id, IMAGE_KEY_PREFIX)
    }
}
