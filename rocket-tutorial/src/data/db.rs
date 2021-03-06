use chrono::{DateTime, Utc};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub hashed_password: String,
    pub salt: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {
    /// 1. 由 20 个随机字母创建 `salt`
    /// 2. 创建 hash 后的密码，并加盐
    /// 3. 由 uuid v4 生成 ID 构建 User
    pub fn new(name: String, email: String, password: String) -> Self {
        let salt: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(20)
            .map(char::from)
            .collect();
        let hashed_password = hash_password(&password, &salt);

        User {
            id: Uuid::new_v4(),
            name,
            email,
            hashed_password,
            salt,
            created: Utc::now(),
            updated: Utc::now(),
        }
    }

    /// 由 InsertableUser 生成 User
    pub fn from_insertable(insertable: InsertableUser) -> Self {
        User::new(insertable.name, insertable.email, insertable.password)
    }

    /// 密码匹配器
    pub fn match_password(&self, password: &str) -> bool {
        argon2::verify_encoded(&self.hashed_password, password.as_bytes()).unwrap()
    }

    /// 密码更新
    pub fn update_password(&mut self, password: &str) {
        self.hashed_password = hash_password(&password.to_owned(), &self.salt);
        self.updated = Utc::now();
    }

    /// 用户更新
    pub fn update_user(&mut self, name: &str, email: &str) {
        self.name = name.to_owned();
        self.email = email.to_owned();
        self.updated = Utc::now();
    }
}

fn hash_password(password: &str, salt: &str) -> String {
    let config = argon2::Config::default();
    argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseUser {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl ResponseUser {
    /// 提取有效返回
    pub fn from_user(user: &User) -> Self {
        ResponseUser {
            id: user.id.to_string(),
            name: format!("{}", user.name),
            email: format!("{}", user.email),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPassword {
    pub password: String,
    pub new_password: Option<String>,
}
