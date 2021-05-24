use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::dao::pg;
use crate::models::STable;

#[derive(Deserialize)]
pub struct CreateTableReq {
    create_if_not_exists: Option<bool>,
}

#[get("/")]
async fn index() -> impl Responder {
    format!("Welcome to Sea Server!")
}

#[post("/table")]
pub async fn create_table(
    table: web::Json<STable>,
    q: web::Query<CreateTableReq>,
    dao: web::Data<pg::DoaPg>,
) -> HttpResponse {
    let create_if_not_exists = q.create_if_not_exists.unwrap_or(false);

    let res = dao.create_table(table.0, create_if_not_exists).await;

    match res {
        Ok(_) => HttpResponse::Ok().body("succeeded"),
        Err(_) => HttpResponse::BadRequest().body("failed"),
    }
}
