use actix_web::{get, post, web, Error, HttpResponse};
use diesel::{r2d2::ConnectionManager, SqliteConnection};
use uuid::Uuid;

use crate::{models, resolvers};

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/// 通过 UID 查询用户
#[get("/user/{user_id}")]
pub async fn get_user(
    pool: web::Data<DbPool>,
    user_uid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user_uid = user_uid.into_inner();
    let conn = pool.get().expect("Couldn't get db connection from pool");

    // 使用 web::block 卸载阻塞的 Diesel 代码，使得 server 的线程不被阻塞
    let user = web::block(move || resolvers::find_user_by_uid(user_uid, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(format!("No user found with uid: {}", user_uid));
        Ok(res)
    }
}

/// 用包含 name 的 form 插入新 user
#[post("/user")]
pub async fn add_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUser>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Couldn't get db connection from pool");

    let user = web::block(move || resolvers::insert_new_user(&form.name, &conn))
        .await
        .map_err(|e| {
            eprint!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(user))
}
