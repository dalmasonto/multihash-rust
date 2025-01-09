use rs_merkle::MerkleTree;
use rs_merkle::{algorithms::Sha256 as Sha2256, Hasher};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

/// Helper function to compute SHA-256 hash and return it as a hex string with prefix "1220".
fn get_hash_sum(content: &str) -> String {
    if content.is_empty() {
        return String::new();
    }
    let mut hasher = Sha256::new();
    hasher.update(content);
    format!("{}", hex::encode(hasher.finalize()))
}

/// Convert a dictionary into a vector of sorted leaves (hashed key-value pairs).
fn dict_to_leaves(data: &BTreeMap<String, String>) -> Vec<String> {
    let leaves: Vec<String> = data
        .iter()
        .map(|(key, value)| get_hash_sum(&format!("{}:{}", key, value)))
        .collect();
    // leaves.sort(); // Ensure the leaves are sorted
    leaves
}

/// Generate a Merkle Tree from the given leaves and return the root hash.
fn merkle_root(mut leaves: Vec<String>) -> String {
    if leaves.is_empty() {
        return String::new();
    }

    while leaves.len() > 1 {
        let mut next_level = Vec::new();
        for chunk in leaves.chunks(2) {
            let concatenated = if chunk.len() == 2 {
                format!("{}{}", chunk[0], chunk[1])
            } else {
                chunk[0].clone()
            };
            // let concatenated_hex = hex::encode(concatenated.as_bytes());
            next_level.push(get_hash_sum(&concatenated));
        }
        leaves = next_level;
        println!("{:?}\n", leaves);
    }

    leaves[0].clone()
}


#[derive(Clone)]
pub struct Sha256Algorithm;

impl Hasher for Sha256Algorithm {
    type Hash = [u8; 32]; // The type of the hash is a fixed-size array of 32 bytes

    fn hash(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new(); // Create a new Sha256 hasher
        hasher.update(data); // Update the hasher with the data
        let result = hasher.finalize(); // Finalize the hashing
        let mut hash_array = [0u8; 32]; // Initialize an array to store the result

        // Copy the result into the array
        hash_array.copy_from_slice(&result);
        
        // hash_array // Return the fixed-size hash array
        result.into()
    }
}

fn main() {
    // Input dictionary (verification data)
    // let mut verification_data = BTreeMap::new();
    // verification_data.insert("previous_verification_hash".to_string(), "".to_string());
    // verification_data.insert(
    //     "nonce".to_string(),
    //     "cAdkH_UCilmrVMI9IGGk4SSdyJoLla1rt3guugbIWRE=".to_string(),
    // );
    // verification_data.insert("domain_id".to_string(), "".to_string());
    // verification_data.insert("local_timestamp".to_string(), "20250109081926".to_string());
    // verification_data.insert("revision_type".to_string(), "file_hash".to_string());
    // verification_data.insert(
    //     "file_hash".to_string(),
    //     "1220902594fb1c28f02f8aec83ea2aec068d24aa104d12712c9c190f888bd21e8ca1".to_string(),
    // );

    let verification_data = vec![
        // ("previous_verification_hash", ""),
        // ("nonce", "cAdkH_UCilmrVMI9IGGk4SSdyJoLla1rt3guugbIWRE="),
        // ("domain_id", ""),
        ("revision_type", "file_hash"),
        ("local_timestamp", "20250109081926"),
        // (
        //     "file_hash",
        //     "1220902594fb1c28f02f8aec83ea2aec068d24aa104d12712c9c190f888bd21e8ca1",
        // ),
    ];

    // // Convert to a BTreeMap to ensure sorted keys
    let mut sorted_data: BTreeMap<String, String> = BTreeMap::new();
    for (key, value) in verification_data {
        sorted_data.insert(key.to_string(), value.to_string());
    }
    println!("Data: {:#?}", sorted_data);
    // // Convert dictionary to leaves
    // let leaves = dict_to_leaves(&sorted_data);
    // // println!("Leaves:");
    // for leaf in &leaves {
    //     println!("  {}", leaf);
    // }

    let leaves = vec![
        "a79244e52894d92742aae0aa2a556774d71149e9c063e457978feae4c93efb9c".to_string(),
        "2c5376b11aac1b4eec4576c0549a676d7789a75b6544e0ae6edd9ea6550b6392".to_string(),
        // "b5a8213f63925eebfb80a719c5c20ee7f698459742f527234afb3bc1c3196299".to_string(),
        // "096813c8b54f2d5d8cd25294bc90876eb2cd4c274bc5c83052d43a702631fc88",
        // "d781acf7ba880ecae581ffd8debcb4f5cb430bc2f237e27a6098471a9f7ffa60",
        // "7eea9613c6543d280b92a70e7b03451901dbf8ed6993eb2488210961277e0775"
    ];

    let byte_leaves: Vec<[u8; 32]> = leaves
        .iter()
        .map(|leaf| {
            // let hex_string = leaf.strip_prefix("1220").unwrap_or(leaf);
            let decoded = hex::decode(leaf).expect("Failed to decode hex string");
            decoded.try_into().expect("Hash is not 32 bytes")
        })
        .collect();

    let merkle_tree = MerkleTree::<Sha2256>::from_leaves(&byte_leaves);
    // println!("=====>>>>>.. 5.2 ");
    let verification_hash = merkle_tree
        .root_hex()
        .unwrap_or(String::from("Error Verification Hash"));
    println!("\nVerification hash: {}", verification_hash);
    // leaves.sort();

    // Generate Merkle Tree root
    // let root = merkle_root(leaves);
    // println!("\nHex root: {}", root);

    // let leaves: Vec<Vec<u8>> = leaves
    //     .iter()
    //     .map(|leaf| hex::decode(leaf).unwrap_or_default())
    //     .collect();

    // let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);
    // println!("=====>>>>>.. 5.2 ");
    // verification_hash = merkle_tree
    //     .root_hex()
    //     .unwrap_or(String::from("Error Verification Hash"));
}
