use actix_web::{App, HttpServer};
use log::info;

use sea_server::dao::pg;
use sea_server::endpoints::db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    let uri = String::from("postgres://postgres:password@localhost/test");
    let dao = pg::DoaPg::new(uri, 5).await;

    info!("Rust Actix Server running... http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .data(dao.clone())
            .service(db::index)
            .service(db::table_create)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
