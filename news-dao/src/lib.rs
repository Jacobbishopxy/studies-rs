mod schema;

#[macro_use]
extern crate diesel;
extern crate news_contract;
// extern crate uuid;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use news_contract::News;
use schema::news::dsl::{desc, id, news, url};

pub async fn connect() -> PgConnection {
    let database_url = String::from("");

    let conn = PgConnection::establish(&database_url);

    match conn {
        Ok(c) => c,
        Err(e) => {
            panic!("Connection error: {:?}", e);
        }
    }
}

pub async fn get_news_by_id(i: &String) -> Option<News> {
    //
    //

    None
}

pub async fn delete_news_by_id(i: &String) -> Option<bool> {
    //
    //

    None
}

pub async fn insert_news(u: &String, d: &String) -> Option<News> {
    //
    //

    None
}

pub async fn list_news() -> Option<Vec<News>> {
    //
    //

    None
}
