use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::dao::pg;
use crate::models::{STable, STableAlter};

#[derive(Deserialize)]
pub struct CreateTableReq {
    create_if_not_exists: Option<bool>,
}

#[get("/")]
async fn index() -> impl Responder {
    format!("Welcome to Sea Server!")
}

#[post("/table_create")]
pub async fn table_create(
    table: web::Json<STable>,
    req: web::Query<CreateTableReq>,
    dao: web::Data<pg::DoaPg>,
) -> HttpResponse {
    let create_if_not_exists = req.create_if_not_exists.unwrap_or(false);

    let res = dao.table_create(table.0, create_if_not_exists).await;

    match res {
        Ok(_) => HttpResponse::Ok().body("succeeded"),
        Err(_) => HttpResponse::BadRequest().body("failed"),
    }
}

#[post("/table_alter")]
pub async fn table_alter(alter: web::Json<STableAlter>, dao: web::Data<pg::DoaPg>) -> HttpResponse {
    let res = dao.table_alter(alter.0).await;

    match res {
        Ok(_) => HttpResponse::Ok().body("succeeded"),
        Err(_) => HttpResponse::BadRequest().body("failed"),
    }
}
