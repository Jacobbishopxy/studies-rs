use actix_web::{delete, get, put, web, HttpResponse, Responder};
use news_dao::DAO;

#[get("/")]
async fn index() -> impl Responder {
    format!("Endpoints: /news")
}

#[get("/news")]
pub async fn list_news(dao: web::Data<DAO>) -> HttpResponse {
    let news = dao.list_news().await;
    HttpResponse::Ok().json(news)
}

#[get("/news/{id}")]
pub async fn get_news_by_id(info: web::Path<String>, dao: web::Data<DAO>) -> HttpResponse {
    let id = &info.as_str();
    let mut new_string = String::new();
    new_string.push_str(id);

    let news = dao.get_news_by_id(&new_string).await;
    HttpResponse::Ok().json(news)
}

#[delete("/news/{id}")]
pub async fn delete_news_by_id(info: web::Path<String>, dao: web::Data<DAO>) -> HttpResponse {
    let id = &info.as_str();
    let mut new_string = String::new();
    new_string.push_str(id);

    let news = dao.delete_news_by_id(&new_string).await;
    HttpResponse::Ok().json(news)
}

#[put("/news/{url}/{desc}")]
pub async fn insert_news(info: web::Path<(String, String)>, dao: web::Data<DAO>) -> impl Responder {
    let url = &info.0 .0;
    let desc = &info.0 .1;
    let news = dao.insert_news(url, desc).await;
    HttpResponse::Ok().json(news)
}
