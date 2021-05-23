extern crate news_contract;

use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, PgConnection};

use news_contract::{schema, News};
use schema::news::dsl::{desc, id, news, url};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct DAO {
    pool: Pool,
}

impl DAO {
    pub fn new(uri: String) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(uri);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        DAO { pool }
    }

    pub async fn get_news_by_id(&self, i: &String) -> Option<News> {
        let conn = &self.pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&i).unwrap();
        let result = news.filter(id.eq(uuid)).first::<News>(conn);

        match result {
            Ok(r) => Some(r),
            Err(_) => None,
        }
    }

    pub async fn delete_news_by_id(&self, i: &String) -> Option<bool> {
        let conn = &self.pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&i).unwrap();
        let result = diesel::delete(news.filter(id.eq(uuid))).execute(conn);

        match result {
            Ok(_) => Some(true),
            Err(_) => None,
        }
    }

    pub async fn insert_news(&self, u: &String, d: &String) -> Option<News> {
        let conn = &self.pool.get().unwrap();
        let result = diesel::insert_into(news)
            .values(&(id.eq(uuid::Uuid::new_v4()), desc.eq(d), url.eq(u)))
            .get_result::<News>(conn);

        match result {
            Ok(n) => Some(n),
            Err(_) => None,
        }
    }

    pub async fn list_news(&self) -> Option<Vec<News>> {
        let conn = &self.pool.get().unwrap();
        let result = news.load::<News>(conn);

        match result {
            Ok(r) => Some(r),
            Err(_) => None,
        }
    }
}
