use streebog::{Streebog512, Digest};

pub fn hash(data: &str) -> String {
    let digest = Streebog512::digest(data.as_bytes());
    hex::encode(digest)
}