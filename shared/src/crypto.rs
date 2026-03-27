use rand::RngExt;
use kuznyechik::Kuznyechik as Cipher;
use cipher::{KeyIvInit, StreamCipher};

use crate::hash;

type Kuznyechik = ctr::Ctr128BE<Cipher>;

const IV_LEN: usize = 16;
const KEY_LEN: usize = 32;

pub fn encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    let mut iv = [0u8; IV_LEN];
    rand::rng().fill(&mut iv);

    let key_hash = hash::hash(key);

    let mut key_bytes = [0u8; KEY_LEN];
    key_bytes.copy_from_slice(&key_hash[..KEY_LEN]);

    let mut kuznyechik = Kuznyechik::new(&key_bytes.into(), &iv.into());
    let mut buffer = data.to_vec();
    kuznyechik.apply_keystream(&mut buffer);

    let mut result = iv.to_vec();
    result.extend_from_slice(&buffer);

    result
}

pub fn decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    let (iv, encrypted) = data.split_at(IV_LEN);

    let key_hash = hash::hash(key);

    let mut key_bytes = [0u8; KEY_LEN];
    key_bytes.copy_from_slice(&key_hash[..KEY_LEN]);

    let mut kuznyechik = Kuznyechik::new(&key_bytes.into(), iv.into());
    let mut result = encrypted.to_vec();
    kuznyechik.apply_keystream(&mut result);

    result
}
