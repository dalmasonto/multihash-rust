use std::collections::HashMap;

use sha2::{Digest, Sha256};

/// Helper function to compute SHA-256 hash and return it as a hex string with prefix "1220".
fn get_hash_sum(content: &str) -> String {
    if content.is_empty() {
        return String::new();
    }
    let mut hasher = Sha256::new();
    hasher.update(content);
    format!("{}", hex::encode(hasher.finalize()))
}

pub fn generate_leaves(verification_data: HashMap<String, String>) -> Vec<String> {
    // Sort the keys and construct the leaves
    let mut sorted_keys: Vec<String> = verification_data.keys().cloned().collect();
    sorted_keys.sort(); // Sort keys alphabetically

    sorted_keys
        .into_iter()
        .filter_map(|key| {
            verification_data
                .get(&key)
                .map(|value| get_hash_sum(format!("{}:{}", key, value).as_str()))
            // Create "key:value"
        })
        .collect()
}

pub fn generate_byte_leaves(leaves: Vec<String>) -> Vec<[u8; 32]> {
    let _leaves: Vec<[u8; 32]> = leaves
        .iter()
        .map(|leaf| {
            let mut array = [0u8; 32];
            let bytes = hex::decode(leaf).unwrap_or_default();
            array.copy_from_slice(&bytes);
            array
        })
        .collect();

    _leaves
}
