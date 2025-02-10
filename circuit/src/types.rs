use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct ProofRequest {
    pub a: String,
    pub b: String,
}

#[derive(Serialize, Clone)]
pub struct ProofResponse {
    pub proof: String,
    pub public_input: String,
}

#[derive(Deserialize, Clone)]
pub struct VerifyRequest {
    pub proof: String,
    pub public_input: String,
}

#[derive(Serialize, Clone)]
pub struct VerifyResponse {
    pub is_valid: bool,
}

/// Struct for the authentication request.
#[derive(Deserialize)]
pub struct AuthRequest {
    pub address: String,
    pub secret_key: String,
}

/// Struct for the Ethereum key pair.
#[derive(Debug)]
pub struct EthKeyPair {
    pub private_key_hex: String,
    pub address_hex: String,
}

/// Struct to hold the ABI and BIN content of contracts.
#[derive(Serialize)]
pub struct ContractInfo {
    pub zk_manager_abi: String,
    pub zk_manager_bin: String,
    pub validator_abi: String,
    pub validator_bin: String,
}
