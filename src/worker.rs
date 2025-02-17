use risc0_zkvm::{default_prover, ExecutorEnv};
use serde::Serialize;
use std::env;

// Define the result structure to hold the result and proof.
#[derive(Serialize)]
pub struct FibResult {
    pub result: u64,
    pub proof: String,
}

// Embed the guest ELF.
const GUEST_ELF: &[u8] = include_bytes!("GUEST.elf");

// Default IMAGE_ID if not provided via an environment variable.
const DEFAULT_IMAGE_ID: &str = "7fbf2a9c7f59f4695fd21c52ed4836fe08558e91704d7d0020ce5bf71bc069bf";

/// Retrieves the IMAGE_ID from an environment variable at runtime,
/// falling back to the default value if necessary. The hex string is then
/// decoded into a 32-byte array.
fn get_image_id() -> [u8; 32] {
    let image_id_str = env::var("IMAGE_ID").unwrap_or_else(|_| DEFAULT_IMAGE_ID.to_string());
    let decoded = hex::decode(image_id_str).expect("Invalid hex string for IMAGE_ID");
    let mut image_id = [0u8; 32];
    image_id.copy_from_slice(&decoded[..32]);
    image_id
}

pub fn phi(a: u64) -> Result<FibResult, String> {
    // Build the executor environment and write the inputs.
    let env = ExecutorEnv::builder()
        .write(&a)
        .map_err(|e| format!("Failed to write input a: {}", e))?
        .build()
        .map_err(|e| format!("Failed to build environment: {}", e))?;

    // Create the prover instance.
    let prover = default_prover();
    let receipt = prover
        .prove(env, GUEST_ELF)
        .map_err(|e| format!("Failed to execute guest: {}", e))?
        .receipt;

    // Decode the guest's output (the result).
    let result: u64 = receipt
        .journal.decode()
        .map_err(|e| format!("Failed to decode output: {}", e))?;

    // Verify the receipt using the IMAGE_ID obtained from the environment.
    let image_id = get_image_id();
    receipt.verify(image_id)
        .map_err(|e| format!("Receipt verification failed: {}", e))?;

    // Serialize the receipt into bytes and encode it as a hex string.
    let receipt_bytes = bincode::serialize(&receipt)
        .map_err(|e| format!("Failed to serialize receipt: {}", e))?;
    let proof = hex::encode(receipt_bytes);

    Ok(FibResult { result, proof })
}
