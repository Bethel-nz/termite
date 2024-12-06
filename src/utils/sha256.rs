use super::HashingUtil;
use sha2::{Digest, Sha256};

pub struct Sha256Hasher {}

impl HashingUtil for Sha256Hasher {
    fn hash_string(&self, input: &str, key: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        hasher.update(key.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
