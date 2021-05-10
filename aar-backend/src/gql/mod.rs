pub mod mutations;
pub mod queries;

use actix_web::{web, HttpResponse, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{Request, Response};

use crate::dbs::mysql::my_pool;
use crate::gql::queries::QueryRoot;

type ActixSchema =
    Schema<queries::QueryRoot, async_graphql::EmptyMutation, async_graphql::EmptySubscription>;

pub async fn build_schema() -> ActixSchema {
    let my_pool = my_pool().await;

    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(my_pool)
        .finish()
}

pub async fn graphql(schema: web::Data<ActixSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
        )))
}
