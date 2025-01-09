import { MerkleTree } from "merkletreejs"
import { sha256 } from "multihashes-sync/sha2"
import { bytes } from 'multiformats'

const dict2Leaves = (obj) => {
    let sorted_leaves = Object.keys(obj).sort();
    console.log("Sorted Leaves: ", sorted_leaves)
    return sorted_leaves  // MUST be sorted for deterministic Merkle tree
        .map((key) => getHashSum(`${key}:${obj[key]}`))
}

// TODO in the Rust version, you should infer what the hashing algorithm
// and the digest size are from the multihash itself. Instead of assuming that
// it is SHA2-256
function getHashSum(content) {
    return content === "" ? "" : bytes.toHex(sha256.digest(content).bytes)
}

let verificationData = {
    // "previous_verification_hash": "",
    // "nonce": "cAdkH_UCilmrVMI9IGGk4SSdyJoLla1rt3guugbIWRE=",
    // "domain_id": "",
    "local_timestamp": "20250109081926",
    "revision_type": "file_hash",
    // "file_hash": "1220902594fb1c28f02f8aec83ea2aec068d24aa104d12712c9c190f888bd21e8ca1",
}


// Merklelize the dictionary
// const leaves = dict2Leaves(verificationData)

let leaves = [
    "1220a79244e52894d92742aae0aa2a556774d71149e9c063e457978feae4c93efb9c",
    "12202c5376b11aac1b4eec4576c0549a676d7789a75b6544e0ae6edd9ea6550b6392",
    "1220b5a8213f63925eebfb80a719c5c20ee7f698459742f527234afb3bc1c3196299",
    // "1220096813c8b54f2d5d8cd25294bc90876eb2cd4c274bc5c83052d43a702631fc88",
    // "1220d781acf7ba880ecae581ffd8debcb4f5cb430bc2f237e27a6098471a9f7ffa60",
    // "12207eea9613c6543d280b92a70e7b03451901dbf8ed6993eb2488210961277e0775"
];

console.log("leaves  json ==>", leaves);

const tree = new MerkleTree(leaves.map(leaf => leaf.slice(4)), getHashSum)

console.log(`Hex root: ${tree.getHexRoot()}`)
