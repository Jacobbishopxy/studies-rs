use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::{prelude::*, PgConnection};
use serde::Deserialize;

use crate::errors::ServiceError;
use crate::models::{Invitation, Pool};
use crate::schema;

#[derive(Deserialize)]
pub struct InvitationData {
    pub email: String,
}

/// 传递给 server route 的主函数
/// invitation_data：外部传递给 server 的 json 数据
/// pool：（在 main.rs 中所设定）由 actix-web 服务所提供，用于任何需求它的 handler
/// Diesel 的请求不是异步的所以我们需要 `web::block` 来执行同步代码，并仍然在父方法中返回一个 future
pub async fn post_invitation(
    invitation_data: web::Json<InvitationData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    // 执行 diesel 的阻塞代码
    let res = web::block(move || create_invitation(invitation_data.into_inner().email, pool)).await;

    match res {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

fn create_invitation(eml: String, pool: web::Data<Pool>) -> Result<(), ServiceError> {
    let invitation = dbg!(query(eml, pool)?);

    Ok(())
    // send_invitation(&invitation)
}

/// diesel query
fn query(eml: String, pool: web::Data<Pool>) -> Result<Invitation, ServiceError> {
    use schema::invitations::dsl::invitations;

    let new_invitation: Invitation = eml.into();
    let conn: &PgConnection = &pool.get().unwrap();

    let inserted_invitation = diesel::insert_into(invitations)
        .values(&new_invitation)
        .get_result(conn)?;

    Ok(inserted_invitation)
}
