pub mod filters;
mod handlers;
pub mod models;

#[cfg(test)]
mod tests {

    use std::collections::HashSet;
    use warp::http::StatusCode;
    use warp::test::request;

    use super::filters;
    use super::models;

    #[tokio::test]
    async fn try_list() {
        use std::str;

        let simulation1 = models::Simulation {
            id: 1,
            name: String::from("The Big Goodbye!"),
        };

        let simulation2 = models::Simulation {
            id: 2,
            name: String::from("Bride Of Chaotica!"),
        };

        let db = models::new_db();

        // 由于我们使用的是 `tokio::sync::Mutex` 而不是 `std::sync::Mutex`
        // (前者是后者 Async 形式的一种实现)，
        // 因此 `lock()` 返回的是一个 future。
        db.lock().await.insert(simulation1.clone());
        db.lock().await.insert(simulation2.clone());

        let api = filters::list_sims(db);

        // Bytes 是 warp 的 RequestBuilder 所处理的 HTML body 内容
        let response = request().method("GET").path("/holodeck").reply(&api).await;

        // 把 bytes 转换成 u8 类型的向量
        let result: Vec<u8> = response.into_body().into_iter().collect();
        // 转换成字符 slice
        let result = str::from_utf8(&result).unwrap();
        // 使用 `serde_json::from_str` 转换为 Simulation 的 HashSet
        let result: HashSet<models::Simulation> = serde_json::from_str(result).unwrap();

        assert_eq!(models::get_simulation(&result, 1).unwrap(), &simulation1);
        assert_eq!(models::get_simulation(&result, 2).unwrap(), &simulation2);

        let response = request()
            .method("GET")
            .path("/holodeck/2")
            .reply(&api)
            .await;

        let result: Vec<u8> = response.into_body().into_iter().collect();
        let result = str::from_utf8(&result).unwrap();
        let result: HashSet<models::Simulation> = serde_json::from_str(result).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(models::get_simulation(&result, 2).unwrap(), &simulation2);
    }

    /// `POST`发送一个请求给服务器用于插入数据。数据以JSON形式发送。
    /// 该功能便是解析（反序列化）JSON，并存储信息（仅在内存中，作者在本项目中不涉及ORM）
    #[tokio::test]
    async fn try_create() {
        let db = models::new_db();
        let api = filters::post_sim(db);

        let response = request()
            .method("POST")
            .path("/holodeck")
            .json(&models::Simulation {
                id: 1,
                name: String::from("The Big Goodbye"),
            })
            .reply(&api)
            .await;

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    /// 测试如果创建重复数据是否会冲突。
    #[tokio::test]
    async fn try_create_duplicates() {
        let db = models::new_db();
        let api = filters::post_sim(db);

        let response = request()
            .method("POST")
            .path("/holodeck")
            .json(&models::Simulation {
                id: 1,
                name: String::from("Bride of Chaotica!"),
            })
            .reply(&api)
            .await;

        assert_eq!(response.status(), StatusCode::CREATED);

        let response = request()
            .method("POST")
            .path("/holodeck")
            .json(&models::Simulation {
                id: 1,
                name: String::from("Bride of Chaotica!"),
            })
            .reply(&api)
            .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    /// 测试更新内容，以及测试状态码：
    /// - 201 创建
    /// - 200 更新
    #[tokio::test]
    async fn try_update() {
        let db = models::new_db();
        let api = filters::update_sim(db);

        let response = request()
            .method("PUT")
            .path("/holodeck/1")
            .json(&models::NewName {
                name: String::from("The Big Goodbye"),
            })
            .reply(&api)
            .await;

        assert_eq!(response.status(), StatusCode::CREATED);

        let response = request()
            .method("PUT")
            .path("/holodeck/1")
            .json(&models::NewName {
                name: String::from("The Short Hello"),
            })
            .reply(&api)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn try_delete() {
        let simulation = models::Simulation {
            id: 1,
            name: String::from("The Big Goodbye!"),
        };

        let db = models::new_db();
        db.lock().await.insert(simulation);

        let api = filters::delete_sim(db);

        let response = request()
            .method("DELETE")
            .path("/holodeck/1")
            .reply(&api)
            .await;

        assert_eq!(response.status(), StatusCode::OK);
    }
}
