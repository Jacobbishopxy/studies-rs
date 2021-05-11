use serde::{Deserialize, Serialize};

#[rbatis::crud_enable(table_name: "users")]
#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(complex)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub cred: String,
}

#[async_graphql::ComplexObject]
impl User {
    pub async fn from(&self) -> String {
        format!("{}<{}>", &self.username, &self.email)
    }
}

#[rbatis::crud_enable(table_name: "users")]
#[derive(async_graphql::InputObject, Serialize, Deserialize, Clone, Debug)]
pub struct NewUser {
    #[graphql(skip)]
    pub id: i32,
    pub email: String,
    pub username: String,
    #[graphql(skip)]
    pub cred: String,
}
