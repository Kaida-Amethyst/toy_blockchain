// Sha256 is a hashing algorithm
use crypto::digest::Digest;
use ring::digest::{Context, SHA256};
use ring::rand::SystemRandom;
use ring::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING};
use std::iter::repeat;

pub fn sha256_digest(data: &[u8]) -> Vec<u8> {
    let mut context = Context::new(&SHA256);
    context.update(data);
    let digest = context.finish();
    digest.as_ref().to_vec()
}

// vec<u8> to hex string
pub fn hex_encode(data: &Vec<u8>) -> String {
    let mut hex = String::from("0x");
    if data.len() == 0 {
        hex.push_str("0");
        return hex;
    }
    for byte in data {
        hex.push_str(&format!("{:02x}", byte));
    }
    hex
}

pub fn new_key_pair() -> Vec<u8> {
    let rng = SystemRandom::new();
    let pkcs8 = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng).unwrap();
    pkcs8.as_ref().to_vec()
}

pub fn ripemd160_digest(data: &[u8]) -> Vec<u8> {
    let mut ripemd160 = crypto::ripemd160::Ripemd160::new();
    ripemd160.input(data);
    let mut buf: Vec<u8> = repeat(0).take(ripemd160.output_bytes()).collect();
    ripemd160.result(&mut buf);
    buf
}

pub fn base58_encode(data: &[u8]) -> String {
    bs58::encode(data).into_string()
}
