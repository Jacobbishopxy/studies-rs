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
    let uuid = uuid::Uuid::parse_str(&i).unwrap();
    let conn = connect().await;
    let result = news.filter(id.eq(uuid)).first::<News>(&conn);

    match result {
        Ok(r) => Some(r),
        Err(_) => None,
    }
}

pub async fn delete_news_by_id(i: &String) -> Option<bool> {
    let uuid = uuid::Uuid::parse_str(&i).unwrap();
    let conn = connect().await;
    let result = diesel::delete(news.filter(id.eq(uuid))).execute(&conn);

    match result {
        Ok(_) => Some(true),
        Err(_) => None,
    }
}

pub async fn insert_news(u: &String, d: &String) -> Option<News> {
    //
    // todo: Insertable

    None
}

pub async fn list_news() -> Option<Vec<News>> {
    let conn = connect().await;
    let result = news.load::<News>(&conn);

    match result {
        Ok(r) => Some(r),
        Err(_) => None,
    }
}
