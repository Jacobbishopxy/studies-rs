use diesel::prelude::*;
use uuid::Uuid;

use crate::models;
use crate::schema;

/// 使用 Diesel 执行查询（通过 uid 查找 user）
pub fn find_user_by_uid(
    uid: Uuid,
    conn: &SqliteConnection,
) -> Result<Option<models::User>, diesel::result::Error> {
    use schema::users::dsl::*;

    let user = users
        .filter(id.eq(uid.to_string()))
        .first::<models::User>(conn)
        .optional()?;

    Ok(user)
}

/// 使用 Diesel 插入一个新数据并返回值
pub fn insert_new_user(
    nm: &str,
    conn: &SqliteConnection,
) -> Result<models::User, diesel::result::Error> {
    use schema::users::dsl::*;

    let new_user = models::User {
        id: Uuid::new_v4().to_string(),
        name: nm.to_owned(),
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(new_user)
}
