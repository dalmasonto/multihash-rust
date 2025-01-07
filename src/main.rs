use hex::encode;
use multihash::{Multihash};

const SHA2_256: u64 = 0x12;

fn generate_hash(data: &[u8]) -> String{
    let hash = Multihash::<256>::wrap(SHA2_256, data);
    let result = hash.unwrap();
    let formatted_hash = format!("{:x}{:x}{}", result.code(), result.size(), hex::encode(result.digest()));
    return hex::encode(result.digest());
}

pub fn get_hash_sum(content: &str) -> String {
    if content.is_empty() {
        return String::new();
    }

    // Using multihash with SHA2-256
    const SHA2_256: u64 = 0x12;
    let hash = Multihash::<64>::wrap(SHA2_256, content.as_bytes())
        .expect("Failed to create multihash");
    
    // Convert digest to hex string
    hex::encode(hash.digest())
}

fn main() {
    let nonce_hash = get_hash_sum("nonce:dzcPaymhMVWKBPcYnPLtgI6oqpDSZlGMBDYzQtTK2");
    let local_timestamp_hash = get_hash_sum("local_timestamp:20250106124826");
    println!("{:?}", nonce_hash);
    println!("{:?}", local_timestamp_hash);
}
