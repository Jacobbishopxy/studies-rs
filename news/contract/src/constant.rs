use std::collections::HashMap;

lazy_static::lazy_static! {
    pub static ref CFG: HashMap<&'static str, String> = {

        dotenv::from_path("news-contract/.env").ok();

        let mut map = HashMap::new();

        map.insert(
            "PG_URI",
            dotenv::var("PG_URI").expect("Expected PG_URI to be set in env!"),
        );

        map
    };
}
