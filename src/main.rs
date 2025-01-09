use rs_merkle::MerkleTree;
use rs_merkle::{algorithms::Sha256 as Sha2256, Hasher};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap};

pub mod utils;
use utils::{generate_byte_leaves, generate_leaves};

fn get_hash_sum(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(hex::decode(input).unwrap_or_else(|_| Vec::new())); // Decode the hex input
    let result = hasher.finalize();
    hex::encode(result) // Return the hash as a hex string
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
// fn merkle_root(mut leaves: Vec<String>) -> String {
//     if leaves.is_empty() {
//         return String::new();
//     }

//     // Ensure all leaves are hex-encoded
//     leaves = leaves
//         .into_iter()
//         .map(|leaf| {
//             if leaf.len() % 2 == 0 {
//                 // Already hex
//                 leaf
//             } else {
//                 // Convert to hex
//                 hex::encode(leaf)
//             }
//         })
//         .collect();

//     while leaves.len() > 1 {
//         let mut next_level = Vec::new();
//         for chunk in leaves.chunks(2) {
//             let concatenated = if chunk.len() == 2 {
//                 format!("{}{}", chunk[0], chunk[1]) // Concatenate two leaves
//             } else {
//                 chunk[0].clone() // Single leaf (odd number of leaves)
//             };
//             next_level.push(get_hash_sum(&concatenated)); // Hash the concatenated string
//         }
//         leaves = next_level;
//         println!("{:?}\n", leaves);
//     }

//     leaves[0].clone()
// }

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
                chunk[0].clone() // If odd, duplicate the last leaf
            };
            let hash = Sha256::digest(concatenated.as_bytes());
            next_level.push(format!("{:x}", hash));
        }
        leaves = next_level;
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

// fn main() {
//     let leaves = vec![
//         "a79244e52894d92742aae0aa2a556774d71149e9c063e457978feae4c93efb9c".to_string(),
//         "2c5376b11aac1b4eec4576c0549a676d7789a75b6544e0ae6edd9ea6550b6392".to_string(),
//         "b5a8213f63925eebfb80a719c5c20ee7f698459742f527234afb3bc1c3196299".to_string(),
//         // "096813c8b54f2d5d8cd25294bc90876eb2cd4c274bc5c83052d43a702631fc88".to_string(),
//         // "d781acf7ba880ecae581ffd8debcb4f5cb430bc2f237e27a6098471a9f7ffa60",
//         // "7eea9613c6543d280b92a70e7b03451901dbf8ed6993eb2488210961277e0775"
//     ];

//     let leaves: Vec<[u8; 32]> = leaves
//         .iter()
//         .map(|leaf| {
//             let mut array = [0u8; 32];
//             let bytes = hex::decode(leaf).unwrap_or_default();
//             array.copy_from_slice(&bytes);
//             array
//         })
//         .collect();

//     // println!("Leaves: {:?}", leaves);

//     let mut merkle_tree = MerkleTree::<Sha256Algorithm>::from_leaves(&leaves);
//     merkle_tree.commit();
//     // println!("=====>>>>>.. 5.2 ");
//     let verification_hash = merkle_tree
//         .root_hex()
//         .unwrap_or(String::from("Error Verification Hash"));
//     println!("Root: {}", verification_hash);

//     // let root = merkle_root(leaves);
//     // println!("Merkle Root: {}", root);
// }

fn main() {
    let verification_data: HashMap<String, String> = [
        ("local_timestamp".to_string(), "20250109081926".to_string()),
        ("revision_type".to_string(), "file_hash".to_string()),
        ("domain_id".to_string(), "".to_string()),
        ("previous_verification_hash".to_string(), "".to_string()),
        (
            "nonce".to_string(),
            "cAdkH_UCilmrVMI9IGGk4SSdyJoLla1rt3guugbIWRE=".to_string(),
        ),
        (
            "file_hash".to_string(),
            "902594fb1c28f02f8aec83ea2aec068d24aa104d12712c9c190f888bd21e8ca1".to_string(),
        ),
    ]
    .into_iter()
    .collect();

    let leaves = generate_leaves(verification_data);
    println!("Leaves: {:#?}", leaves);

    let tree = merkle_root(leaves.clone());

    // println!("Hex root 1: {}", tree);

    // HEX ENCODING WORKS WELL WITH THIS AND SO FAR THE BEST OUTPUT
    let mut merkle_tree = MerkleTree::<Sha2256>::from_leaves(&generate_byte_leaves(leaves.clone()));
    merkle_tree.commit();
    println!("Hex root 2: {}", merkle_tree.root_hex().unwrap());

}


// c89f4492bd1473f0de5e0c0b28987e8d248d02a4d556d4d66913c1da2772a0a1 - Hex encoding
// 6804f580679ef92060c0335f577045d42d077989c7eb93d5887c9b97944a9b22 - UTF 8 encoding
