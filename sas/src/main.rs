mod errors;
mod handlers;
mod models;
mod schema;
mod utils;

#[macro_use]
extern crate diesel;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use handlers::invitation_handler;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_path("sas/.env").ok();
    std::env::set_var(
        "RUST_LOG",
        "simple-auth-server=debug,actix_web=info,actix_server=info",
    );
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // 创建数据库连接池
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: models::Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    // 启动 http 服务
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            // 启用 logger
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(utils::SECRET_KEY.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(domain.as_str())
                    .max_age(86400) // 一天的秒数
                    .secure(false), // 仅 https 时为 true
            ))
            // 限制服务可接收的最大数据
            .data(web::JsonConfig::default().limit(4096))
            // 所有路径都归在 `/api/` route 下
            .service(
                web::scope("/api").service(
                    web::resource("/invitation")
                        .route(web::post().to(invitation_handler::post_invitation)),
                ),
                // .service(
                //     web::resource("/register/{invitation_id}")
                //         .route(web::post().to(register_handler::register_user)),
                // )
                // .service(
                //     web::resource("/auth")
                //         .route(web::post().to(auth_handler::login))
                //         .route(web::delete().to(auth_handler::logout))
                //         .route(web::get().to(auth_handler::get_me)),
                // ),
            )
    })
    .bind("127.0.0.1:3030")?
    .run()
    .await
}
