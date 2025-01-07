

fn create_witness_revision() -> Revision {
    Revision {
        previous_verification_hash: None,
        nonce: None,
        domain_id: None,
        local_timestamp: None,
        revision_type: None,
        file_hash: None,
        link_type: None,
        link_require_indepth_verification: None,
        link_verification_hash: None,
        link_uri: None,
        signature: None,
        signature_public_key: None,
        signature_wallet_address: None,
        signature_type: None,
        witness_merkle_root: None,
        witness_timestamp: None,
        witness_network: None,
        witness_smart_contract_address: None,
        witness_transaction_hash: None,
        witness_sender_account_address: None,
        witness_merkle_proof: None,
        leaves: Vec::new(),
    }
}