use ring::error::Unspecified;
use ring::{digest, pbkdf2};
use std::num::NonZeroU32;

const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
type PasswordHash = [u8; CREDENTIAL_LEN];

/// encryption helper
/// n_iter: iteration for pbkdf2 derive
/// salt: secret key
pub struct EncryptionHelper {
    n_iter: NonZeroU32,
    salt: String,
}

impl EncryptionHelper {
    /// create a new helper
    pub fn new(n: u32, salt: String) -> Self {
        EncryptionHelper {
            n_iter: NonZeroU32::new(n).unwrap(),
            salt,
        }
    }

    fn hash_to_string(hash: PasswordHash) -> String {
        let mut res = String::from("");
        for i in hash.iter() {
            res.push(*i as char)
        }

        res
    }

    fn string_to_hash(string: String) -> PasswordHash {
        let mut res = [0u8; CREDENTIAL_LEN];
        for (i, c) in string.chars().enumerate() {
            res[i] = c as u8;
        }

        res
    }

    /// encrypt string
    pub fn hash(&self, password: &str) -> String {
        let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA512,
            self.n_iter,
            self.salt.as_bytes(),
            password.as_bytes(),
            &mut pbkdf2_hash,
        );
        EncryptionHelper::hash_to_string(pbkdf2_hash)
    }

    /// verify password
    pub fn verify(&self, password: &str, hashed_str: String) -> Result<(), Unspecified> {
        let previously_derived = EncryptionHelper::string_to_hash(hashed_str);

        pbkdf2::verify(
            pbkdf2::PBKDF2_HMAC_SHA512,
            self.n_iter,
            self.salt.as_bytes(),
            password.as_bytes(),
            &previously_derived as &[u8],
        )
    }
}
