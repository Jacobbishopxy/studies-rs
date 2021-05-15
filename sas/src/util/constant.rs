use std::collections::HashMap;

lazy_static::lazy_static! {
    pub static ref CFG: HashMap<&'static str, String> = {

        dotenv::from_path("sas/.env").ok();

        let mut map = HashMap::new();

        map.insert(
            "SECRET_KEY",
            dotenv::var("SECRET_KEY").expect("Expected SECRET_KEY to be set in env!"),
        );

        map.insert(
            "SMTP_USERNAME",
            dotenv::var("SMTP_USERNAME").expect("Expected SMTP_USERNAME to be set in env!"),
        );

        map.insert(
            "SMTP_PASSWORD",
            dotenv::var("SMTP_PASSWORD").expect("Expected SMTP_PASSWORD to be set in env!"),
        );

        map.insert(
            "SMTP_HOST",
            dotenv::var("SMTP_HOST").expect("Expected SMTP_ to be set in env!"),
        );

        map.insert(
            "SMTP_PORT",
            dotenv::var("SMTP_PORT").expect("Expected SMTP_ to be set in env!"),
        );

        map
    };
}
