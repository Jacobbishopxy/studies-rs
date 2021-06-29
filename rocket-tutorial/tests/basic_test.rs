use lazy_static;
mod common;

#[cfg(test)]
mod test {
    use rocket::http::{ContentType, Status};

    use super::common;

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
        assert_eq!(
            resp.body_string(),
            Some("{\"status\":\"Success\",\"message\":\"List of users\"}".into())
        );
    }

    #[test]
    fn new_user_rt_test() {
        let client = common::setup();
        let mut resp = client.post("/api/users").dispatch();
        assert_eq!(resp.status(), Status::Ok);
        assert_eq!(resp.content_type(), Some(ContentType::JSON));
        assert_eq!(
            resp.body_string(),
            Some("{\"status\":\"Success\",\"message\":\"Creation of new user\"}".into())
        );
    }

    #[test]
    fn info_user_rt_test() {
        let client = common::setup();
        let mut resp = client.get("/api/users/1").dispatch();
        assert_eq!(resp.status(), Status::Ok);
        assert_eq!(resp.content_type(), Some(ContentType::JSON));
        assert_eq!(
            resp.body_string(),
            Some("{\"status\":\"Success\",\"message\":\"Info for user 1\"}".into())
        );
    }

    #[test]
    fn update_user_rt_test() {
        let client = common::setup();
        let mut resp = client.put("/api/users/1").dispatch();
        assert_eq!(resp.status(), Status::Ok);
        assert_eq!(resp.content_type(), Some(ContentType::JSON));
        assert_eq!(
            resp.body_string(),
            Some("{\"status\":\"Success\",\"message\":\"Update info for user 1\"}".into())
        );
    }

    #[test]
    fn delete_user_rt_test() {
        let client = common::setup();
        let mut resp = client.delete("/api/users/1").dispatch();
        assert_eq!(resp.status(), Status::Ok);
        assert_eq!(resp.content_type(), Some(ContentType::JSON));
        assert_eq!(
            resp.body_string(),
            Some("{\"status\":\"Success\",\"message\":\"Delete user 1\"}".into())
        );
    }
}
