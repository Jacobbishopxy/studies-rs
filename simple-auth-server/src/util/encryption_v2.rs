use crate::error::ServiceError;
use crate::util::{EncryptionHelper, CFG};

/// 密码加密
pub fn hash_password(password: &str) -> Result<String, ServiceError> {
    let salt = CFG.get("SECRET_KEY").unwrap();
    let n = CFG.get("SECRET_LEN").unwrap().parse::<u32>().unwrap();
    let ecp = EncryptionHelper::new(n, salt.to_string());

    Ok(ecp.hash(password))
}

/// 密码验证
/// hash: 存于数据库的加密密码
/// password: 接口传入的用户密码
pub fn verify(hash: &str, password: &str) -> Result<bool, ServiceError> {
    let salt = CFG.get("SECRET_KEY").unwrap();
    let n = CFG.get("SECRET_LEN").unwrap().parse::<u32>().unwrap();
    let ecp = EncryptionHelper::new(n, salt.to_string());

    match ecp.verify(password, hash.to_string()) {
        Ok(_) => {
            println!("Verify succeed");
            Ok(true)
        }
        Err(e) => {
            println!("Verify failed: {:#?}", e);
            Ok(false)
        }
    }
}
