use super::HashingUtil;

pub struct Blake3Hasher {}

impl HashingUtil for Blake3Hasher {
    fn hash_string(&self, input: &str, key: &str) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(input.as_bytes());
        hasher.update(key.as_bytes());
        format!("{}", hasher.finalize().to_hex())
    }
}
