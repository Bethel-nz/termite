pub mod blake3;
pub mod md5;
pub mod sha256;

pub trait HashingUtil {
    fn hash_string(&self, input: &str, key: &str) -> String;
}

pub fn get_hasher(algorithm: &str) -> Box<dyn HashingUtil> {
    match algorithm {
        "SHA256" => Box::new(sha256::Sha256Hasher {}),
        "BLAKE3" => Box::new(blake3::Blake3Hasher {}),
        "MD5" => Box::new(md5::Md5Hasher {}),
        _ => Box::new(sha256::Sha256Hasher {}),
    }
}
