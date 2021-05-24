use sqlx::postgres::PgQueryResult;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::adaptors::sea_adaptor;
use crate::models::STable;

#[derive(Clone)]
pub struct DoaPg {
    pool: Pool<Postgres>,
}

impl DoaPg {
    pub async fn new(uri: String, max_conn: u32) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(max_conn)
            .connect(&uri)
            .await
            .expect("Connection Error!");

        DoaPg { pool }
    }

    pub async fn create_table(
        &self,
        table: STable,
        create_if_not_exists: bool,
    ) -> Result<PgQueryResult, sqlx::Error> {
        let query = sea_adaptor::table_create(&table, create_if_not_exists);
        sqlx::query(&query).execute(&self.pool).await
    }
}
