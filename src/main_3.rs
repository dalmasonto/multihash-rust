use hex; use rs_merkle::MerkleTree;
// Hex decoding
use sha2::{Digest, Sha256};

// Placeholder for your custom algorithm if needed
struct Sha2256;

// impl merkletree::hash::Algorithm<[u8; 32]> for Sha2256 {
//     fn hash(&mut self, data: &[u8]) -> [u8; 32] {
//         let hash = Sha256::digest(data);
//         hash.try_into().expect("Hash output is not 32 bytes")
//     }

//     fn reset(&mut self) {}
// }

fn main() {
    // Example JSON leaves as hex strings
    let leaves = vec![
        "12202873eeea87385d54a0d8b195176f63ff108126c260e54751f42f5f2931023c02",
        "12207eea9613c6543d280b92a70e7b03451901dbf8ed6993eb2488210961277e0775",
    ];

    // Convert the hex strings into Vec<[u8; 32]>
    let byte_leaves: Vec<[u8; 32]> = leaves
        .iter()
        .map(|leaf| {
            let decoded = hex::decode(leaf).expect("Failed to decode hex string");
            decoded.try_into().expect("Decoded hash is not 32 bytes")
        })
        .collect();

    // Create the Merkle Tree
    let merkle_tree = MerkleTree::<Sha2256>::from_leaves(&byte_leaves);

    println!("Merkle tree created successfully!");
}
