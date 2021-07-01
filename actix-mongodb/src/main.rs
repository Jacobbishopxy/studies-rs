use std::env;
use std::sync::Mutex;

use actix_web::{web, App, HttpServer, Responder};
use mongodb::{options::ClientOptions, Client};

pub mod logs_handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");

    dotenv::dotenv().ok();

    let mongo_url = dotenv::var("CONNECTION_STRING_LOGS")
        .expect("Expected CONNECTION_STRING_LOGS  to be set in env!");

    let mut client_options = ClientOptions::parse(&mongo_url).await.unwrap();
    client_options.app_name = Some("PlantApi".to_string());
    let client = web::Data::new(Mutex::new(Client::with_options(client_options).unwrap()));

    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .service(web::scope("/api").configure(logs_handlers::scoped_config))
            .route("/", web::get().to(hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn hello() -> impl Responder {
    format!("Hello fellow Rustacean!")
}
