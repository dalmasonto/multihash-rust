// use hex::encode;
// // use merkletree::merkle::MerkleTree;
// use multihash::{Multihash, MultihashGeneric};
// use rs_merkle::{Hasher, MerkleTree};
use sha2::{Digest, Sha256 as Sha2256};

use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};

const SHA2_256: u64 = 0x12;

// fn generate_hash(data: &[u8]) -> String {
//     let hash = Multihash::<256>::wrap(SHA2_256, data);
//     let result = hash.unwrap();
//     let formatted_hash = format!(
//         "{:x}{:x}{}",
//         result.code(),
//         result.size(),
//         hex::encode(result.digest())
//     );
//     return hex::encode(result.digest());
// }

pub fn get_hash_sum(content: &str) -> String {
    if content.is_empty() {
        String::new()
    } else {
        let mut hasher = Sha2256::new();
        hasher.update(content.as_bytes());
        // let res = hasher.finalize();
        // res
        format!("1220{:x}", hasher.finalize())
    }
}

#[derive(Clone, Debug)]
pub struct Sha256Hasher;

const HASH_SIZE: usize = 32; // Hash output size in bytes
impl Hasher for Sha256Hasher {
    type Hash = [u8; 32]; // Hash output size

    fn hash(data: &[u8]) -> Self::Hash {
        let mut hasher = Sha2256::new();
        hasher.update(data);
        let result = hasher.finalize();
        result.try_into().expect("Hash should be 32 bytes")
    }

    fn concat_and_hash(left: &Self::Hash, right: Option<&Self::Hash>) -> Self::Hash {
        let mut concatenated: Vec<u8> = (*left).into();

        match right {
            Some(right_node) => {
                let mut right_node_clone: Vec<u8> = (*right_node).into();
                concatenated.append(&mut right_node_clone);
                Self::hash(&concatenated)
            }
            None => *left,
        }
    }
}

fn main() {
    // let nonce_hash = get_hash_sum("nonce:dzcPaymhMVWKBPcYnPLtgI6oqpDSZlGMBDYzQtTK2-o");
    // let local_timestamp_hash = get_hash_sum("local_timestamp:20250106124826");

    // let loong_text_hash = get_hash_sum("Education in its true meaning is deeper than an ocean, wiser than truth and majestic than words. It has the power to vanish the darkness, cruelty and incivility from our society. In Arabic, it means to discover reality. In Latin, it is to bring up, bring out, bring forth what is within and to lead. Many schools, religious leaders, philosophers and great people owe education for their success and happiness in their lives. Furthermore, education is the holly gift which we can give to our future generation. It opens a multitude of opportunities for people to progress in every battle of life. Education is a sight for the blind, a sense of life, a medicine for suffering and an elixir of immortality. The people who know it's worth are destined to a wealth of blessings.");
    // println!("{:?}", nonce_hash);
    // println!("{:?}", local_timestamp_hash);
    // println!("{:?}", loong_text_hash);

    let mut leaf_values = vec![
        "1220a79244e52894d92742aae0aa2a556774d71149e9c063e457978feae4c93efb9c",
        "12202c5376b11aac1b4eec4576c0549a676d7789a75b6544e0ae6edd9ea6550b6392",
        "12202873eeea87385d54a0d8b195176f63ff108126c260e54751f42f5f2931023c02",
        "12202199ded8938bf652ca9d2e1c09022e94b24f8bb84b23937ad5ed5cc86a0fc160",
        "1220d781acf7ba880ecae581ffd8debcb4f5cb430bc2f237e27a6098471a9f7ffa60",
        "12207eea9613c6543d280b92a70e7b03451901dbf8ed6993eb2488210961277e0775",
    ];

    leaf_values.sort();

    let leaves: Vec<[u8; 32]> = leaf_values
    .iter()
    .map(|x| Sha256::hash(x.as_bytes()))
    .collect();

    // Create a Merkle Tree
    // let mut merkle_tree = MerkleTree::<Sha256Hasher>::new();
    // for leaf in &leaves_bytes {
    //     merkle_tree.insert(leaf);
    // }

    let mut merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);

    // Finalize the tree
    merkle_tree.commit();
    // let hex_msg = merkle_tree.root_hex();

    // println!("Id: {:?}", );

    // Get the Merkle Root
    let root = merkle_tree.root().expect("Tree should have a root");

    println!("Merkle Root: {}, {}", hex::encode(root), get_hash_sum(merkle_tree.root_hex().unwrap().as_str()));
}
