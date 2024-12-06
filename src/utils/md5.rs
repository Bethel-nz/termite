use super::HashingUtil;
use md5::compute;

pub struct Md5Hasher {}

impl HashingUtil for Md5Hasher {
    fn hash_string(&self, input: &str, key: &str) -> String {
        let combined = format!("{}{}", input, key);
        let result = compute(combined.as_bytes());
        format!("{:x}", result)
    }
}
