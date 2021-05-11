#[macro_use]
extern crate diesel;

use actix_web::{middleware, App, HttpServer};
use diesel::{r2d2::ConnectionManager, SqliteConnection};

mod controllers;
mod models;
mod resolvers;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::from_path("ads/.env").ok();

    // 设置数据库连接池
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let bind = "127.0.0.1:8080";

    println!("Starting server at: {}", &bind);

    // 启动 HTTP 服务
    HttpServer::new(move || {
        App::new()
            // 建立被用于 web::Data<Pool> extractor 的连接池
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(controllers::get_user)
            .service(controllers::add_user)
    })
    .bind(&bind)?
    .run()
    .await
}
