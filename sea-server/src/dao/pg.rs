use sqlx::postgres::PgQueryResult;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::adaptors::sea_adaptor;
use crate::models::{STable, STableAlter};

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

    pub async fn table_create(
        &self,
        table: STable,
        create_if_not_exists: bool,
    ) -> Result<PgQueryResult, sqlx::Error> {
        let query = sea_adaptor::table_create(&table, create_if_not_exists);
        sqlx::query(&query).execute(&self.pool).await
    }

    pub async fn table_alter(&self, alter: STableAlter) -> Result<PgQueryResult, sqlx::Error> {
        let vec_query = sea_adaptor::table_alter(&alter);

        let mut tx = self.pool.begin().await.expect("Transaction start");

        for query in vec_query {
            if let e @ Err(_) = sqlx::query(&query).execute(&mut tx).await {
                return e;
            }
        }

        match tx.commit().await {
            Ok(_) => Ok(PgQueryResult::default()),
            Err(_) => Err(sqlx::Error::WorkerCrashed),
        }
    }
}
