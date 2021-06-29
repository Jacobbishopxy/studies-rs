#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_attributes)]

#[macro_use]
use rocket::*;
use rocket_contrib::helmet::SpaceHelmet;
use rocket_contrib::serve::StaticFiles;
use std::sync::{Arc, Mutex};

pub mod data;
mod routes;

pub struct Users {
    pub db: Arc<Mutex<Vec<data::db::User>>>,
}
impl Users {
    pub fn new() -> Self {
        Users {
            db: Arc::new(Mutex::new(vec![])),
        }
    }
}

pub fn rocket_builder() -> Rocket {
    ignite()
        .attach(SpaceHelmet::default())
        .mount("/", routes![routes::ping::ping_fn])
        .mount("/", routes![routes::echo::echo_fn])
        .mount(
            "/api",
            routes![
                routes::user::user_list_rt,
                routes::user::new_user_rt,
                routes::user::info_user_rt,
                routes::user::update_user_rt,
                routes::user::delete_user_rt,
                routes::user::patch_user_rt,
                routes::user::id_user_rt,
            ],
        )
        .mount("/files", StaticFiles::from("static/"))
        .manage(Users::new())
}
