use async_graphql::Context;
use rbatis::rbatis::Rbatis;

use crate::users::{self, models::User};
use crate::util::constant::GqlResult;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn all_users(&self, ctx: &Context<'_>) -> GqlResult<Vec<User>> {
        let my_pool = ctx.data_unchecked::<Rbatis>();
        users::services::all_users(my_pool).await
    }
}
