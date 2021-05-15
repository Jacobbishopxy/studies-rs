use argon2::{self, Config};

use crate::error::ServiceError;
use crate::util::CFG;

const SALT: &'static [u8] = b"supersecuresalt";

// 警告！ 以下仅适合 Demo，在生产环境中则需要对此进行更多的研究
pub fn hash_password(password: &str) -> Result<String, ServiceError> {
    let secret = CFG.get("SECRET_KEY").unwrap().as_bytes();
    let config = Config {
        secret,
        ..Default::default()
    };

    argon2::hash_encoded(password.as_bytes(), &SALT, &config).map_err(|err| {
        dbg!(err);
        ServiceError::InternalServerError
    })
}

pub fn verify(hash: &str, password: &str) -> Result<bool, ServiceError> {
    let secret = CFG.get("SECRET_KEY").unwrap().as_bytes();
    argon2::verify_encoded_ext(hash, password.as_bytes(), secret, &[]).map_err(|err| {
        dbg!(err);
        ServiceError::Unauthorized
    })
}

// 你可能注意到了函数的返回都是通过一个 `Result` 或 `map_error()` 来包装自定义的 Error。
// 因为这样可以在之后的调用中使用 `?` 操作符（另一种转换错误的方法是为返回的 error 值实现 `From` 特性）
