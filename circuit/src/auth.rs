use crate::files;
use crate::types::EthKeyPair;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::result::Result;

/// Files where we'll store the hex-encoded private key and address.
const PRIVATE_KEY_FILE: &str = "/tmp/eth_sk";
const ADDRESS_FILE: &str = "/tmp/eth_addr";

/// Saves the given private key and address (as hex strings) to disk.
///
/// # Example
///
/// ```
/// use crate::auth::{save_keypair, load_keypair, EthKeyPair};
///
/// let keypair = EthKeyPair {
///     private_key_hex: "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string(),
///     address_hex: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string(),
/// };
///
/// save_keypair(&keypair).expect("Failed to save keypair");
/// let loaded_keypair = load_keypair();
/// assert_eq!(loaded_keypair.private_key_hex, keypair.private_key_hex);
/// assert_eq!(loaded_keypair.address_hex, keypair.address_hex);
/// ```
pub fn save_keypair(kp: &EthKeyPair) -> Result<(), String> {
    // 1) Save the private key to `/tmp/eth_sk`
    {
        let mut sk_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(PRIVATE_KEY_FILE)
            .map_err(|e| e.to_string())?;
        sk_file
            .write_all(kp.private_key_hex.as_bytes())
            .map_err(|e| e.to_string())?;
    }

    // 2) Save the address to `/tmp/eth_addr`
    {
        let mut addr_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(ADDRESS_FILE)
            .map_err(|e| e.to_string())?;
        addr_file
            .write_all(kp.address_hex.as_bytes())
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// Loads the keypair from disk. If the files don't exist or cannot
/// be read, returns empty strings without failing.
///
/// # Example
///
/// ```
/// use crate::auth::load_keypair;
///
/// let keypair = load_keypair();
/// assert_eq!(keypair.private_key_hex, "");
/// assert_eq!(keypair.address_hex, "");
/// ```
pub fn load_keypair() -> EthKeyPair {
    // If either file does not exist, return empty
    if !Path::new(PRIVATE_KEY_FILE).exists() || !Path::new(ADDRESS_FILE).exists() {
        return EthKeyPair {
            private_key_hex: "".to_string(),
            address_hex: "".to_string(),
        };
    }

    // Attempt to load each file; if there's any error, return empty
    let private_key_hex = match files::read_file_to_string(PRIVATE_KEY_FILE) {
        Ok(contents) => contents,
        Err(_) => "".to_string(),
    };

    let address_hex = match files::read_file_to_string(ADDRESS_FILE) {
        Ok(contents) => contents,
        Err(_) => "".to_string(),
    };

    // Return the struct
    EthKeyPair {
        private_key_hex,
        address_hex,
    }
}

/// Sets the credentials (address and secret key).
///
/// # Example
///
/// ```
/// use crate::auth::{set_credentials, get_public_address};
///
/// let address = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string();
/// let secret_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string();
/// set_credentials(address.clone(), secret_key.clone()).expect("Failed to set credentials");
/// assert_eq!(get_public_address(), Some(address));
/// ```
pub fn set_credentials(address: String, secret_key: String) -> Result<(), String> {
    let keypair = EthKeyPair {
        private_key_hex: secret_key,
        address_hex: address,
    };
    save_keypair(&keypair)
}

/// Gets the public address of the credentials.
///
/// # Example
///
/// ```
/// use crate::auth::{set_credentials, get_public_address};
///
/// let address = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string();
/// let secret_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string();
/// set_credentials(address.clone(), secret_key.clone()).expect("Failed to set credentials");
/// assert_eq!(get_public_address(), Some(address));
/// ```
pub fn get_public_address() -> Option<String> {
    let keypair = load_keypair();
    if keypair.address_hex.is_empty() {
        None
    } else {
        Some(keypair.address_hex)
    }
}
