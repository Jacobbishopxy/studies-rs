mod endpoints;

extern crate actix_web;
extern crate contract;
extern crate env_logger;

use actix_web::{App, HttpServer};
use log::info;

use contract::constant::CFG;
use dao::DAO;
use endpoints::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    let dao = DAO::new(CFG.get("PG_URI").unwrap().to_string());

    info!("Rust Actix Server running... http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .data(dao.clone())
            .service(index)
            .service(list_news)
            .service(insert_news)
            .service(get_news_by_id)
            .service(delete_news_by_id)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
