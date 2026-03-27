use streebog::{Streebog512, Digest};

pub fn hash(data: &[u8]) -> Vec<u8> {
    Streebog512::digest(data)
        .to_vec()
}
