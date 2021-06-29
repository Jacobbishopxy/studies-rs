#![feature(proc_macro_hygiene, decl_macro)]

use rocket::*;
use rocket_contrib::helmet::SpaceHelmet;
use rocket_contrib::serve::StaticFiles;

#[get("/echo/<echo>")]
fn echo_fn(echo: String) -> String {
    format!("{}", echo)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(SpaceHelmet::default()) // 服务安全
        .mount("/", routes![echo_fn])
        .mount("/files", StaticFiles::from("static/"))
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn echo_test() {
        let client = Client::new(rocket()).expect("Valid Rocket instance");
        let mut resp = client.get("/echo/test_echo").dispatch();
        assert_eq!(resp.status(), Status::Ok);
        assert_eq!(resp.body_string(), Some("test_echo".into()));
    }
}
