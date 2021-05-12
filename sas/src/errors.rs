//! 为了将来程序拓展，自定义错误格式
//! 使 http 响应可以携带自定义信息

use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use std::convert::From;
use uuid::Error as ParseError;

// Rust 提供了强大的类型转换工具。
// 在本 app 中我们将操作一些不同的库，例如通过 diesel 存储数据，通过 bcrypt 哈希密码等等。
// 这些操作都有可能带来错误，但是我们需要转换它们成为自定义的错误类型。
// `std::convert::From` 是一个特性允许我们转换（详见官方文档）。
// 通过实现 `From` 特性，我们可以是用 `?` 操作符来传递（propagate）不同类型的错误，再转换为 `ServiceError` 类型

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,
}

/// 为 ServiceError 实现 ResponseError 特性
/// 转换 error 成为 http 的响应
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        }
    }
}

/// 可以在 handlers 的早期返回用户所提供的 UUID 无效的错误
impl From<ParseError> for ServiceError {
    fn from(_: ParseError) -> Self {
        ServiceError::BadRequest("Invalid UUID".into())
    }
}

impl From<DBError> for ServiceError {
    fn from(error: DBError) -> Self {
        // 现在暂且只需要关心 diesel 的 UniqueViolation 错误
        // 随着 app 迭代，这样可以更好的处理未来更多的错误
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return ServiceError::BadRequest(message);
                }
                ServiceError::InternalServerError
            }
            _ => ServiceError::InternalServerError,
        }
    }
}
