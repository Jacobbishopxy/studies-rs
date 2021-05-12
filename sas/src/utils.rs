// use crate::errors::ServiceError;
// use argon2::{self, Config};

lazy_static::lazy_static! {
    pub static ref SECRET_KEY: String = std::env::var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8));
}

const SALT: &'static [u8] = b"supersecuresalt";
