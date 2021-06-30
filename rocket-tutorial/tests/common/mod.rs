use crate::lazy_static::lazy_static;

use rocket::local::Client;
use rocket_tutorial::rocket_builder;

// 仅在 Dev 模式下使用的 client 初始化
pub fn setup() -> &'static Client {
    lazy_static! {
        static ref CLIENT: Client = Client::new(rocket_builder()).expect("Valid Rocket instance");
    }

    &*CLIENT
}
