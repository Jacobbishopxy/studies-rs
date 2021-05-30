mod dbs;
mod gql;
mod users;
mod util;

use actix_web::{guard, web, App, HttpServer};

use gql::{build_schema, graphiql, graphql};
use util::constant::CFG;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = build_schema().await;

    let (address, port, giql) = (
        CFG.get("ADDRESS").unwrap(),
        CFG.get("PORT").unwrap(),
        CFG.get("GIQL_VER").unwrap(),
    );
    println!("GraphQL UI: http://{}:{}", address, port);
    println!("GraphiQL UI: http://{}:{}/{}", address, port, giql);

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(
                web::resource(CFG.get("GQL_VER").unwrap())
                    .guard(guard::Post())
                    .to(graphql),
            )
            .service(
                web::resource(CFG.get("GIQL_VER").unwrap())
                    .guard(guard::Get())
                    .to(graphiql),
            )
    })
    .bind(format!(
        "{}:{}",
        CFG.get("ADDRESS").unwrap(),
        CFG.get("PORT").unwrap()
    ))?
    .run()
    .await
}
