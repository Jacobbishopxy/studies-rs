use lazy_static;
use rocket::http::{ContentType, Status};
use rocket_tutorial::data::db::ResponseUser;
use serde_json;

mod common;

#[cfg(test)]
mod basic_test {
    use super::*;

    #[test]
    fn echo_test() {
        let client = common::setup();
        let mut resp = client.get("/echo/test_echo").dispatch();

        assert_eq!(resp.status(), Status::Ok);
        assert_eq!(resp.body_string(), Some("test_echo".into()));
    }

    #[test]
    fn ping_test() {
        let client = common::setup();
        let mut resp = client.get("/ping").dispatch();

        assert_eq!(resp.status(), Status::Ok);
        assert_eq!(resp.body_string(), Some("PONG!".into()));
    }

    #[test]
    fn user_list_rt_test() {
        let client = common::setup();
        let mut resp = client.get("/api/users").dispatch();

        assert_eq!(resp.status(), Status::Ok);
        assert_eq!(resp.content_type(), Some(ContentType::JSON));

        let mut resp_body = resp.body_string().unwrap();
        resp_body.retain(|c| !c.is_numeric()); // 保留所有非数值字符

        assert_eq!(resp_body, "[]");
    }

    #[test]
    fn new_user_rt_test() {
        let client = common::setup();
        let mut resp = client
            .post("/api/users")
            .header(ContentType::JSON)
            .body(
                r##"{
                    "name": "Jacob Xie",
                    "email": "jx@m.com",
                    "password": "u-guess"
                }"##,
            )
            .dispatch();

        assert_eq!(resp.status(), Status::Ok);
        assert_eq!(resp.content_type(), Some(ContentType::JSON));

        let resp_body = resp.body_string().expect("Response Body");
        let user: ResponseUser =
            serde_json::from_str(&resp_body.as_str()).expect("Valid User Response");

        assert_eq!(user.name, "Jacob Xie");
        assert_eq!(user.email, "jx@m.com");
    }

    #[test]
    fn info_user_rt_test() {
        let client = common::setup();
        let mut resp_new_user = client
            .post("/api/users")
            .header(ContentType::JSON)
            .body(
                r##"{
                    "name": "Jacob Xie",
                    "email": "jx@m.com",
                    "password": "u-guess"
                }"##,
            )
            .dispatch();
        let resp_body = resp_new_user.body_string().expect("Response Body");
        let user_new: ResponseUser =
            serde_json::from_str(&resp_body.as_str()).expect("Valid User Response");
        let id = user_new.id;
        let mut resp = client.get(format!("/api/users/{}", id)).dispatch();
        let resp_body = resp.body_string().expect("Response Body");
        let user: ResponseUser =
            serde_json::from_str(&resp_body.as_str()).expect("Valid User Response");

        assert_eq!(resp.status(), Status::Ok);
        assert_eq!(resp.content_type(), Some(ContentType::JSON));
        assert_eq!(user.name, "Jacob Xie");
        assert_eq!(user.email, "jx@m.com");
        assert_eq!(user.id, id);
    }

    #[test]
    fn update_user_rt_test() {
        let client = common::setup();
        let mut resp_new_user = client
            .post("/api/users")
            .header(ContentType::JSON)
            .body(
                r##"{
                    "name": "Jac X",
                    "email": "jax@m.com",
                    "password": "u-guess"
                }"##,
            )
            .dispatch();
        let resp_body = resp_new_user.body_string().expect("Response Body");
        let user_new: ResponseUser =
            serde_json::from_str(&resp_body.as_str()).expect("Valid User Response");
        let id = user_new.id;
        let mut resp = client
            .put(format!("/api/users/{}", id))
            .header(ContentType::JSON)
            .body(
                r##"{
                    "name": "Jacob Xie",
                    "email": "jx@m.com",
                    "password": "u-guess"
                }"##,
            )
            .dispatch();
        let resp_body = resp.body_string().expect("Response Body");
        let user: ResponseUser =
            serde_json::from_str(&resp_body.as_str()).expect("Valid User Response");
        assert_eq!(resp.status(), Status::Ok);
        assert_eq!(resp.content_type(), Some(ContentType::JSON));
        assert_eq!(user.name, "Jacob Xie");
        assert_eq!(user.email, "jx@m.com");
        assert_eq!(user.id, id);
    }

    #[test]
    fn delete_user_rt_test() {
        let client = common::setup();
        let mut resp_new_user = client
            .post("/api/users")
            .header(ContentType::JSON)
            .body(
                r##"{
                    "name": "Jacob Xie",
                    "email": "jx@m.com",
                    "password": "u-guess"
                }"##,
            )
            .dispatch();
        let resp_body = resp_new_user.body_string().expect("Response Body");
        let user_new: ResponseUser =
            serde_json::from_str(&resp_body.as_str()).expect("Valid User Response");
        let id = user_new.id;
        let mut resp = client
            .delete(format!("/api/users/{}", id))
            .header(ContentType::JSON)
            .body(
                r##"{
                    "password": "u-guess"
                }"##,
            )
            .dispatch();
        let resp_body = resp.body_string().expect("Response Body");
        let user: ResponseUser =
            serde_json::from_str(&resp_body.as_str()).expect("Valid User Response");
        assert_eq!(resp.status(), Status::Ok);
        assert_eq!(resp.content_type(), Some(ContentType::JSON));
        assert_eq!(user.name, "Jacob Xie");
        assert_eq!(user.email, "jx@m.com");
        assert_eq!(user.id, id);
    }
}
