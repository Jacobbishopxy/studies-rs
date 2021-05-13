use crate::errors::ServiceError;
use argon2::{self, Config};

lazy_static::lazy_static! {
    pub static ref SECRET_KEY: String = std::env::var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8));
}

const SALT: &'static [u8] = b"supersecuresalt";

// 警告！ 以下仅适合 Demo，在生产环境中则需要对此进行更多的研究
pub fn hash_password(password: &str) -> Result<String, ServiceError> {
    let config = Config {
        secret: SECRET_KEY.as_bytes(),
        ..Default::default()
    };
    argon2::hash_encoded(password.as_bytes(), &SALT, &config).map_err(|err| {
        dbg!(err);
        ServiceError::InternalServerError
    })
}

pub fn verify(hash: &str, password: &str) -> Result<bool, ServiceError> {
    argon2::verify_encoded_ext(hash, password.as_bytes(), SECRET_KEY.as_bytes(), &[]).map_err(
        |err| {
            dbg!(err);
            ServiceError::Unauthorized
        },
    )
}

// 你可能注意到了函数的返回都是通过一个 `Result` 或 `map_error()` 来包装自定义的 Error。
// 因为这样可以在之后的调用中使用 `?` 操作符（另一种转换错误的方法是为返回的 error 值实现 `From` 特性）
