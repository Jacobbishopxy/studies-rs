mod migration;

use contract::constant::CFG;
use migration::{create_table_news, CreateTableNewsMigration, NewsMigration};
use sqlx::postgres::PgPoolOptions;

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let database_url = CFG.get("PG_URI").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    let s = CreateTableNewsMigration::new().gen();

    let result = create_table_news(&pool, &s).await;

    match result {
        Ok(r) => println!("OK: {:#?}", r),
        Err(e) => println!("Err: {:#?}", e),
    }

    Ok(())
}
