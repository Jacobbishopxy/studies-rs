//! 构建 Auth 的意义在于拥有一种方式来验证从 authenticated client 发送而来的请求。
//! Actix-web 拥有有一个 `FromRequest` 特性可被任何类型实现并使用，其目的在于提取请求中的数据。

use actix_identity::Identity;
use actix_web::{
    dev::Payload, error::BlockingError, web, Error, FromRequest, HttpRequest, HttpResponse,
};
use diesel::prelude::*;
use diesel::PgConnection;
use futures::future::{err, ok, Ready};
use serde::Deserialize;

use crate::error::ServiceError;
use crate::model::{Pool, SlimUser, User};
use crate::schema;
use crate::util::verify;

/// 由 client 发送包含 email 和 password 的结构体
/// 使用 email 来提取数据库中的用户，使用 verify 函数来匹配密码
/// 如果全部通过，由 `id.remember(serialized_user)` 设置 cookie
#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

// 简单的别名使得目的明确，以及可读性
pub type LoggedUser = SlimUser;

impl FromRequest for LoggedUser {
    type Config = ();
    type Error = Error;
    type Future = Ready<Result<LoggedUser, Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        if let Ok(identity) = Identity::from_request(req, payload).into_inner() {
            if let Some(user_json) = identity.identity() {
                if let Ok(user) = serde_json::from_str(&user_json) {
                    return ok(user);
                }
            }
        }
        err(ServiceError::Unauthorized.into())
    }
}

pub async fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Ok().finish()
}

pub async fn login(
    auth_data: web::Json<AuthData>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || query(auth_data.into_inner(), pool)).await;

    match res {
        Ok(user) => {
            let user_string = serde_json::to_string(&user).unwrap();
            id.remember(user_string);
            Ok(HttpResponse::Ok().finish())
        }
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

pub async fn get_me(logged_user: LoggedUser) -> HttpResponse {
    HttpResponse::Ok().json(logged_user)
}

/// Diesel query
fn query(auth_data: AuthData, pool: web::Data<Pool>) -> Result<SlimUser, ServiceError> {
    use schema::users::dsl::{email, users};
    let conn: &PgConnection = &pool.get().unwrap();
    let mut items = users
        .filter(email.eq(&auth_data.email))
        .load::<User>(conn)?;

    if let Some(user) = items.pop() {
        if let Ok(matching) = verify(&user.hash, &auth_data.password) {
            if matching {
                return Ok(user.into());
            }
        }
    }
    Err(ServiceError::Unauthorized)
}
