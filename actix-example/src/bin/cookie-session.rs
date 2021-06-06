//! Example of cookie based session
//! Session data is stored in cookie, it is limited to 4kb
//!
//! [Redis session example](https://github.com/actix/examples/tree/master/redis-session)
//!
//! [User guide](https://actix.rs/docs/middleware/#user-sessions)

use actix_session::{CookieSession, Session};
use actix_web::{middleware::Logger, web, App, HttpRequest, HttpServer, Result};

/// 简易带有 session 的 index 句柄
async fn index(session: Session, req: HttpRequest) -> Result<&'static str> {
    println!("{:?}", req);

    // RequestSession 特性用于访问 session
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        counter = count + 1;
        session.set("counter", counter)?;
    } else {
        session.set("counter", counter)?;
    }

    Ok("welcome!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    println!("Starting http server: 127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            // 开启 logger
            .wrap(Logger::default())
            // 开启 session middleware
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
