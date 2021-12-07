//!

mod broadcaster;
mod db;
mod dto;
mod errors;
mod model;
mod rds;
mod services;

use mongodb::Client as MongoClient;
use redis::Client as RedisClient;
use std::{env, sync::Arc};

use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env.local").ok();
    env_logger::init();

    info!("Starting MongoDB & Redis demo server");

    // mongo 客户端
    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI env var should be specified");
    let mut client_options = mongodb::options::ClientOptions::parse(&mongodb_uri)
        .await
        .unwrap();
    client_options.app_name = Some("mongodb-redis-demo".to_owned());
    let mongodb_client = MongoClient::with_options(client_options).unwrap();

    // redis 客户端
    let redis_uri = env::var("REDIS_URI").expect("REDIS_URI env var should be specified");
    let redis_client = RedisClient::open(redis_uri).expect("Can't create Redis client");
    let redis_connection_manager = redis_client
        .get_tokio_connection_manager()
        .await
        .expect("Can't create Redis connection manager");

    // 业务服务
    // let planet_service = Arc::new(PlanetService::new(
    //     mongodb_client,
    //     redis_client,
    //     redis_connection_manager.clone(),
    // ));
    // let rate_limiting_service = Arc::new(RateLimitingService::new(redis_connection_manager));

    Ok(())
}
