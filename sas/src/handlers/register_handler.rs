use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::{insert_into, prelude::*};
use serde::Deserialize;

use crate::errors::ServiceError;
use crate::models::{Invitation, Pool, SlimUser, User};
use crate::schema;
use crate::utils::hash_password;

// UserData 用于从客户端的一个 post 请求中提取数据
#[derive(Debug, Deserialize)]
pub struct UserData {
    pub email: String,
    pub password: String,
}

/// 注册用户的流程与邀请注册类似。
/// 客户端在特定的 path 上发送数据，我们验证 invitation 后从 plain text 上创建一个哈希密码，
/// 并存储于数据库中，最后返回 `SlimUser` 的 JSON。
pub async fn register_user(
    invitation_id: web::Path<String>,
    user_data: web::Json<UserData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res =
        web::block(move || query(invitation_id.into_inner(), user_data.into_inner(), pool)).await;

    match res {
        Ok(user) => Ok(HttpResponse::Ok().json(&user)),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

fn query(
    invitation_id: String,
    user_data: UserData,
    pool: web::Data<Pool>,
) -> Result<SlimUser, ServiceError> {
    use schema::invitations::dsl::{email, id, invitations};
    use schema::users::dsl::users;

    let invitation_id = uuid::Uuid::parse_str(&invitation_id)?;

    let conn: &PgConnection = &pool.get().unwrap();
    invitations
        .filter(id.eq(invitation_id))
        .filter(email.eq(&user_data.email))
        .load::<Invitation>(conn)
        .map_err(|_db_error| ServiceError::BadRequest("Invalid Invitation".into()))
        .and_then(|mut result| {
            if let Some(invitation) = result.pop() {
                // 如果 invitation 没有过期
                if invitation.expires_at > chrono::Local::now().naive_local() {
                    // 尝试哈希密码，否则返回错误并转换成 ServiceError
                    let password: String = hash_password(&user_data.password)?;
                    let user = User::from_details(invitation.email, password);
                    let inserted_user: User = insert_into(users).values(&user).get_result(conn)?;

                    dbg!(&inserted_user);

                    return Ok(inserted_user.into());
                }
            }
            Err(ServiceError::BadRequest("Invalid Invitation".into()))
        })
}
