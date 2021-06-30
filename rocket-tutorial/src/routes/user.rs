use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Response};
use rocket::*;
use rocket::{response, State};
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::uuid::Uuid;

use crate::data::db::{InsertableUser, ResponseUser, User, UserPassword};
use crate::Users;

#[derive(Debug)]
pub struct ApiResponse {
    status: Status,
    message: JsonValue,
}

impl ApiResponse {
    pub fn ok(message: JsonValue) -> Self {
        ApiResponse {
            status: Status::Ok,
            message,
        }
    }

    pub fn err(message: JsonValue) -> Self {
        ApiResponse {
            status: Status::InternalServerError,
            message,
        }
    }
}

/// 自定义的 Response 转为 Rocket 的 Responder
impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, request: &Request) -> response::Result<'r> {
        Response::build_from(self.message.respond_to(&request).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

#[get("/users")]
pub fn user_list_rt(userdb: State<Users>) -> ApiResponse {
    let v = userdb.db.lock().unwrap();
    let users = &*v;

    ApiResponse::ok(json!([users.len()]))
}

#[post("/users", format = "json", data = "<user>")] // 数据格式：json；数据类型：user
pub fn new_user_rt(userdb: State<Users>, user: Json<InsertableUser>) -> ApiResponse {
    let mut v = userdb.db.lock().unwrap();
    let users = &mut *v;
    users.push(User::from_insertable((*user).clone()));

    ApiResponse::ok(json!(ResponseUser::from_user(&users.last().unwrap())))
}

#[get("/users/<id>")]
pub fn info_user_rt(userdb: State<Users>, id: Uuid) -> ApiResponse {
    let mut v = userdb.db.lock().unwrap();
    let users = &mut *v;
    let pos = users // 通过 id 遍历查找 user
        .iter()
        .position(|x| x.id.to_string() == id.to_string());

    match pos {
        Some(p) => ApiResponse::ok(json!(ResponseUser::from_user(&v[p]))),
        None => ApiResponse::err(json!(format!("id {} not found", id))),
    }
}

/// 更新用户名或者用户邮箱
#[put("/users/<id>", format = "json", data = "<user>")]
pub fn update_user_rt(userdb: State<Users>, user: Json<InsertableUser>, id: Uuid) -> ApiResponse {
    let mut v = userdb.db.lock().unwrap();
    let users = &mut *v;
    let pos = users
        .iter()
        .position(|x| x.id.to_string() == id.to_string());
    match pos {
        Some(p) => {
            if v[p].match_password(&user.password) {
                v[p].update_user(&user.name, &user.email);
                ApiResponse::ok(json!(ResponseUser::from_user(&v[p])))
            } else {
                ApiResponse::err(json!("user not authenticated"))
            }
        }
        None => ApiResponse::err(json!(format!("id {} not found", id))),
    }
}

#[delete("/users/<id>", format = "json", data = "<user>")]
pub fn delete_user_rt(userdb: State<Users>, user: Json<UserPassword>, id: Uuid) -> ApiResponse {
    let mut v = userdb.db.lock().unwrap();
    let users = &mut *v;
    let pos = users
        .iter()
        .position(|x| x.id.to_string() == id.to_string());
    match pos {
        Some(p) => {
            if v[p].match_password(&user.password) {
                let u = v[p].clone();
                v.remove(p);
                ApiResponse::ok(json!(ResponseUser::from_user(&u)))
            } else {
                ApiResponse::err(json!("user not authenticated"))
            }
        }
        None => ApiResponse::err(json!(format!("id {} not found", id))),
    }
}

/// 更新用户密码
#[patch("/users/<id>", format = "json", data = "<user>")] // 使用 patch 更新独立项；使用 put 更新若干项
pub fn patch_user_rt(userdb: State<Users>, user: Json<UserPassword>, id: Uuid) -> ApiResponse {
    let mut v = userdb.db.lock().unwrap();
    let users = &mut *v;
    let pos = users
        .iter()
        .position(|x| x.id.to_string() == id.to_string());
    match pos {
        Some(p) => {
            if v[p].match_password(&user.password) {
                match &user.new_password {
                    Some(passw) => {
                        v[p].update_password(&passw);
                        ApiResponse::ok(json!("Password updated"))
                    }
                    None => ApiResponse::err(json!("Password not provided")),
                }
            } else {
                ApiResponse::err(json!("user not authenticated"))
            }
        }
        None => ApiResponse::err(json!(format!("id {} not found", id))),
    }
}

/// 通过用户邮箱获取用户信息
#[get("/users/<email>", rank = 2)] // 由于与 `/users/<id>` 同路径，使用 rank = 2 意为解析 id 失效后的方法
pub fn id_user_rt(userdb: State<Users>, email: String) -> ApiResponse {
    let mut v = userdb.db.lock().unwrap();
    let users = &mut *v;
    let pos = users.iter().position(|x| x.email == email);
    match pos {
        Some(p) => ApiResponse::ok(json!(ResponseUser::from_user(&v[p]))),
        None => ApiResponse::err(json!(format!("user {} not found", email))),
    }
}
