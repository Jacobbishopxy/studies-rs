use actix_identity::Identity;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use rand::Rng;

async fn index(id: Identity) -> String {
    format!(
        "Hello {}",
        id.identity().unwrap_or_else(|| "Anonymous".to_owned())
    )
}

async fn login(id: Identity) -> HttpResponse {
    id.remember("user1".to_owned());
    HttpResponse::Found().header("location", "/").finish()
}

async fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Found().header("location", "/").finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // 生成一个随机的 32 字节钥匙。注意每个项目使用一个唯一的钥匙是非常重要的。
    // 任何有钥匙访问权的人可以为其它用户生成验证 cookie。
    let private_key = rand::thread_rng().gen::<[u8; 32]>();
    HttpServer::new(move || {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&private_key)
                    .name("auth-example")
                    .secure(false),
            ))
            // 开启 logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/login").route(web::post().to(login)))
            .service(web::resource("/logout").to(logout))
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
