import { MerkleTree } from "merkletreejs"
import { sha256 } from "multihashes-sync/sha2"
import { bytes } from 'multiformats'
import crypto from 'crypto'


const sha256Hasher = (data) => crypto.createHash('sha256').update(data).digest('hex');

const dict2Leaves = (obj) => {
    let sorted_leaves = Object.keys(obj).sort();
    return sorted_leaves  // MUST be sorted for deterministic Merkle tree
        .map((key) => getHashSum(`${key}:${obj[key]}`))
}

// TODO in the Rust version, you should infer what the hashing algorithm
// and the digest size are from the multihash itself. Instead of assuming that
// it is SHA2-256
function getHashSum(content) {
    return content === "" ? "" : bytes.toHex(sha256.digest(content).bytes)
}

// function createHexBuffer(hexString){
//     const buffer = Buffer.from(hexString, 'hex');
//     return buffer
// }

let verificationData = {
    "previous_verification_hash": "",
    "nonce": "cAdkH_UCilmrVMI9IGGk4SSdyJoLla1rt3guugbIWRE=",
    "file_hash": "902594fb1c28f02f8aec83ea2aec068d24aa104d12712c9c190f888bd21e8ca1",
    "domain_id": "",
    "local_timestamp": "20250109081926",
    "revision_type": "file_hash",
}



// Merklelize the dictionary
const leaves = dict2Leaves(verificationData)
console.log("Leaves: ", JSON.stringify(leaves, null, 4))

const tree = new MerkleTree(leaves.map(leaf => leaf.slice(4)), getHashSum, {
    duplicateOdd: false,
})
// console.log(`Hex root 1: ${tree.getHexRoot()};`)

// This with hex 2 of rust seem to match
const tree2 = new MerkleTree(leaves.map(leaf => leaf.slice(4)), sha256Hasher, {
    duplicateOdd: false,
})

console.log(`Hex root 2: ${tree2.getHexRoot()}`)


