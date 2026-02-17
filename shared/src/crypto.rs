use rand::Rng;
use kuznyechik::Kuznyechik;
use cipher::{KeyIvInit, StreamCipher};

use crate::hash;

type KuznyechikCtr = ctr::Ctr128BE<Kuznyechik>;

pub fn encrypt(data: &Vec<u8>, key: &str) -> String {
    let mut iv = [0u8; 16];
    rand::thread_rng().fill(&mut iv);

    let key_hash = hash::hash(key);
    let key_real = hex::decode(key_hash).unwrap();

    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(&key_real[..32]);

    let mut cipher = KuznyechikCtr::new(&key_bytes.into(), &iv.into());
    let mut buffer = data.to_vec();
    cipher.apply_keystream(&mut buffer);

    let mut encrypted = iv.to_vec();
    encrypted.extend_from_slice(&buffer);

    hex::encode(encrypted)
}

pub fn decrypt(data: &Vec<u8>, key: &str) -> Vec<u8> {
    let decoded = hex::decode(data).unwrap();
    let (iv, ciphertext) = decoded.split_at(16);

    let key_hash = hash::hash(key);
    let key_real = hex::decode(key_hash).unwrap();

    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(&key_real[..32]);

    let mut cipher = KuznyechikCtr::new(&key_bytes.into(), iv.into());
    let mut buffer = ciphertext.to_vec();
    cipher.apply_keystream(&mut buffer);

    buffer
}