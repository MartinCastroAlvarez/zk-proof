use crate::auth::load_keypair;
use crate::files;
use crate::security::get_verifier_params;
use ethers::abi::Abi;
use ethers::contract::ContractFactory;
use ethers::core::types::Address;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Middleware, Provider};
use ethers::signers::{LocalWallet, Signer};
use log::error;
use log::info;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

const CONTRACT_ADDRESS_FILE: &str = "/tmp/contract_addr";

/// Saves the given contract address (as a hex string) to disk.
///
/// # Example
///
/// ```
/// use crate::contracts::{save_contract_address, load_contract_address};
///
/// let contract_address = "0x1234567890abcdef1234567890abcdef12345678".to_string();
/// save_contract_address(&contract_address).expect("Failed to save contract address");
/// let loaded_address = load_contract_address().expect("Failed to load contract address");
/// assert_eq!(loaded_address, contract_address);
/// ```
pub fn save_contract_address(address: &str) -> Result<(), String> {
    let mut addr_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(CONTRACT_ADDRESS_FILE)
        .map_err(|e| e.to_string())?;
    addr_file
        .write_all(address.as_bytes())
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Loads the contract address from disk. If the file doesn't exist or is empty,
/// returns an error.
///
/// # Example
///
/// ```
/// use crate::contracts::load_contract_address;
///
/// let loaded_address = load_contract_address().expect("Failed to load contract address");
/// assert!(!loaded_address.is_empty());
/// ```
pub fn load_contract_address() -> Result<String, std::io::Error> {
    if !Path::new(CONTRACT_ADDRESS_FILE).exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Contract address file does not exist",
        ));
    }

    let address_hex = files::read_file_to_string(CONTRACT_ADDRESS_FILE)?;
    if address_hex.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Contract address file is empty",
        ));
    }

    Ok(address_hex)
}
