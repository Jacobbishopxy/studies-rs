use std::sync::Mutex;

use actix_web::{web, HttpResponse, Responder};
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, from_bson, oid::ObjectId, Bson, DateTime, Document},
    options::FindOptions,
    Client,
};
use serde::{Deserialize, Serialize};

// db 名称
const MONGO_DB: &'static str = "ioPlantDB";
// collection 名称
const MONGO_COLL_LOGS: &'static str = "logs";

#[derive(Deserialize)]
pub struct NewLog {
    pub id: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(rename = "deviceId")]
    pub device_id: String,
    pub message: String,
}

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/logs")
            .route(web::get().to(get_logs))
            .route(web::post().to(add_log)),
    );
}

async fn get_logs(data: web::Data<Mutex<Client>>) -> impl Responder {
    let logs_collection = data
        .lock()
        .unwrap()
        .database(MONGO_DB)
        .collection::<Document>(MONGO_COLL_LOGS);

    // 没有 filter 即查询全部
    let filter = doc! {};
    let find_options = FindOptions::builder().sort(doc! {"_id": -1}).build();
    let mut cursor = logs_collection.find(filter, find_options).await.unwrap();

    let mut results = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let log: Log = from_bson(Bson::Document(document)).unwrap();
                results.push(log);
            }
            _ => {
                return HttpResponse::InternalServerError().finish();
            }
        }
    }

    HttpResponse::Ok().json(serde_json::json!(results))
}

async fn add_log(data: web::Data<Mutex<Client>>, new_log: web::Json<NewLog>) -> impl Responder {
    let logs_collection = data
        .lock()
        .unwrap()
        .database(MONGO_DB)
        .collection::<Document>(MONGO_COLL_LOGS);

    let data = doc! {"deviceId": &new_log.id, "message": &new_log.message, "createdOn": Bson::DateTime(DateTime::now())};

    match logs_collection.insert_one(data, None).await {
        Ok(db_result) => {
            if let Some(new_id) = db_result.inserted_id.as_object_id() {
                println!("New document inserted with id {}", new_id);
            };
            return HttpResponse::Created().json(db_result.inserted_id);
        }
        Err(err) => {
            println!("Failed! {}", err);
            return HttpResponse::InternalServerError().finish();
        }
    }
}
