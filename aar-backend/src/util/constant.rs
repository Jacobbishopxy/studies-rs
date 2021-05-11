use lazy_static::lazy_static;
use std::collections::HashMap;

pub type GqlResult<T> = std::result::Result<T, async_graphql::Error>;

lazy_static! {
    // 定义在 .env 文件中的变量
    pub static ref CFG: HashMap<&'static str, String> = {

        // 如果在项目目录执行 `cargo run -p aar-backend`：
        dotenv::from_path("aar-backend/.env").ok();

        // 如果在本子项目目录执行 `cargo run`：
        // dotenv::dotenv().ok();

        let mut map = HashMap::new();

        map.insert(
            "ADDRESS",
             dotenv::var("ADDRESS").expect("Expected ADDRESS to be set in env!")
        );
        map.insert(
            "PORT",
            dotenv::var("PORT").expect("Expected PORT to be set in env!"),
        );
        map.insert(
            "GQL_VER",
            dotenv::var("GQL_VER").expect("Expected GQL_VER to be set in env!"),
        );
        map.insert(
            "GIQL_VER",
            dotenv::var("GIQL_VER").expect("Expected GIQL_VER to be set in env!"),
        );
        map.insert(
            "MYSQL_URI",
            dotenv::var("MYSQL_URI").expect("Expected MYSQL_URI to be set in env!"),
        );

        map
    };
}
