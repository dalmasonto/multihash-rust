
pub struct BasicRevision {
    pub previous_verification_hash: String,
    pub nonce: String,
    pub domain_id: String,
    pub local_timestamp: Timestamp,
    pub revision_type: String,
    pub file_hash: Hash,
}

pub struct LinkRevision {
    pub link_type: String,
    pub link_require_indepth_verification: bool,
    pub link_verification_hash: Hash,
    pub link_uri: String,
}

pub struct WitnessRevision {
    pub witness_merkle_root: String,
    pub witness_timestamp: Timestamp,
    pub witness_network: String,
    pub witness_smart_contract_address: String,
    pub witness_transaction_hash: Hash,
    pub witness_sender_account_address: String,
    pub witness_merkle_proof: String,
}
