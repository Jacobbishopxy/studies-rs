extern crate barrel;

use barrel::backend::Pg;
use barrel::migration::Migration;
use barrel::types;
use sqlx::{postgres, Postgres};

pub trait NewsMigration {
    fn new() -> Self;
    fn gen(&self) -> String;
}

pub struct CreateTableNewsMigration {}
impl NewsMigration for CreateTableNewsMigration {
    fn new() -> Self {
        CreateTableNewsMigration {}
    }

    fn gen(&self) -> String {
        let mut m = Migration::new();
        m.create_table_if_not_exists("news", |t| {
            t.add_column("id", types::uuid().primary(true));
            t.add_column("desc", types::text());
            t.add_column("url", types::text());
        });
        m.make::<Pg>().to_owned()
    }
}

pub async fn create_table_news(
    pool: &sqlx::Pool<Postgres>,
    query_str: &str,
) -> Result<postgres::PgQueryResult, sqlx::Error> {
    sqlx::query(query_str).execute(pool).await
}
