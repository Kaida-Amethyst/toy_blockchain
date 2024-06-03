// Sha256 is a hashing algorithm
use ring::digest::{Context, SHA256};

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
