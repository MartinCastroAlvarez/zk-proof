use ark_bn254::{Bn254, Fq, Fr};
use ark_groth16::{
    generate_random_parameters, prepare_verifying_key, PreparedVerifyingKey, ProvingKey,
    VerifyingKey,
};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use rand::thread_rng;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

use ark_ff::{BigInteger, PrimeField};
use ethers::types::U256;

use crate::circuit::PrimerCircuit;

const PK_FILE: &str = "/tmp/pkx";
const VK_FILE: &str = "/tmp/vkx";

/// Converts a `PreparedVerifyingKey` to a Base64 encoded string.
///
/// # Examples
/// ```
/// # use ark_bn254::Bn254;
/// # use ark_groth16::{ProvingKey, prepare_verifying_key, generate_random_parameters};
/// # use ark_ff::Field;
/// # use ark_std::test_rng;
/// # use crate::vk::to_base64;
/// # let rng = &mut test_rng();
/// # let params = generate_random_parameters::<Bn254, _, _>(crate::circuit::ExampleCircuit { a: None }, rng).unwrap();
/// # let pvk = prepare_verifying_key(&params.vk);
/// let base64_string = to_base64(&pvk);
/// assert!(!base64_string.is_empty());
/// ```
pub fn to_base64(pvk: &PreparedVerifyingKey<Bn254>) -> String {
    let vk = &pvk.vk;
    let mut vk_bytes = Vec::new();
    vk.serialize(&mut vk_bytes)
        .expect("Failed to serialize verification key");
    BASE64.encode(vk_bytes)
}

/// Deserializes a `PreparedVerifyingKey` from a Base64 encoded string.
///
/// # Examples
/// ```
/// # use ark_bn254::Bn254;
/// # use ark_groth16::{PreparedVerifyingKey, prepare_verifying_key};
/// # use crate::vk::from_base64;
/// # let pvk: PreparedVerifyingKey<Bn254> = /* obtain from somewhere */;
/// let base64_string = to_base64(&pvk);
/// let deserialized_pvk = from_base64(&base64_string).unwrap();
/// assert_eq!(pvk, deserialized_pvk);
/// ```
pub fn from_base64(vk_base64: &str) -> Result<PreparedVerifyingKey<Bn254>, String> {
    let vk_bytes = BASE64.decode(vk_base64).map_err(|e| e.to_string())?;
    let vk = VerifyingKey::deserialize(&vk_bytes[..]).map_err(|e| e.to_string())?;
    Ok(prepare_verifying_key(&vk))
}

/// Saves a `PreparedVerifyingKey` to a file in Base64 format.
///
/// # Examples
/// ```
/// # use ark_bn254::Bn254;
/// # use ark_groth16::{generate_random_parameters, PreparedVerifyingKey};
/// # use crate::vk::save_verifying_key;
/// # let pvk: PreparedVerifyingKey<Bn254> = /* obtain from somewhere */;
/// save_verifying_key(&pvk).expect("Failed to save verifying key");
/// ```
pub fn save_verifying_key(pvk: &PreparedVerifyingKey<Bn254>) -> Result<(), String> {
    let vk_base64 = to_base64(pvk);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(VK_FILE)
        .map_err(|e| e.to_string())?;

    file.write_all(vk_base64.as_bytes())
        .map_err(|e| e.to_string())
}

/// Loads a `PreparedVerifyingKey` from a file.
///
/// # Examples
/// ```
/// # use ark_bn254::Bn254;
/// # use crate::vk::load_verifying_key;
/// let pvk = load_verifying_key().expect("Failed to load verifying key");
/// ```
pub fn load_verifying_key() -> Result<PreparedVerifyingKey<Bn254>, String> {
    let mut file = File::open(VK_FILE).map_err(|e| e.to_string())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| e.to_string())?;
    from_base64(&contents).map_err(|e| e.to_string())
}

/// Saves a `ProvingKey` to a file in Base64 format.
///
/// # Examples
/// ```
/// # use ark_bn254::Bn254;
/// # use ark_groth16::{generate_random_parameters, ProvingKey};
/// # use crate::vk::save_proving_key;
/// # let pk: ProvingKey<Bn254> = /* obtain from somewhere */;
/// save_proving_key(&pk).expect("Failed to save proving key");
/// ```
pub fn save_proving_key(pk: &ProvingKey<Bn254>) -> Result<(), String> {
    let mut pk_bytes = Vec::new();
    pk.serialize(&mut pk_bytes).map_err(|e| e.to_string())?;

    let pk_base64 = BASE64.encode(pk_bytes);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(PK_FILE)
        .map_err(|e| e.to_string())?;

    file.write_all(pk_base64.as_bytes())
        .map_err(|e| e.to_string())
}

/// Loads a `ProvingKey` from a file.
///
/// # Examples
/// ```
/// # use ark_bn254::Bn254;
/// # use crate::vk::load_proving_key;
/// let pk = load_proving_key().expect("Failed to load proving key");
/// ```
pub fn load_proving_key() -> Result<ProvingKey<Bn254>, String> {
    let mut file = File::open(PK_FILE).map_err(|e| e.to_string())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| e.to_string())?;

    let pk_bytes = BASE64.decode(contents).map_err(|e| e.to_string())?;
    let pk = ProvingKey::deserialize(&pk_bytes[..]).map_err(|e| e.to_string())?;
    Ok(pk)
}

/// Generates or loads both the `ProvingKey` and `PreparedVerifyingKey`.
///
/// - If `/tmp/pkx` and `/tmp/vkx` exist, load them from disk.
/// - Otherwise, generate fresh keys and save them to disk.
///
/// # Returns
/// A tuple containing the `ProvingKey` and `PreparedVerifyingKey`.
///
/// # Examples
/// ```
/// # use ark_bn254::{Bn254, Fr};
/// # use crate::vk::get_or_create_pvks;
/// let (pk, pvk) = get_or_create_pvks();
/// ```
pub fn get_or_create_pvks() -> (ProvingKey<Bn254>, PreparedVerifyingKey<Bn254>) {
    // Check if both ProvingKey and VerifyingKey files exist
    if Path::new(PK_FILE).exists() && Path::new(VK_FILE).exists() {
        // Load ProvingKey
        let pk = load_proving_key().expect("failed to load pk");
        // Load VerifyingKey
        let pvk = load_verifying_key().expect("failed to load vk");
        (pk, pvk)
    } else {
        // Generate new proving parameters
        let mut rng = thread_rng();

        // Initialize a blank circuit
        let blank_circuit = PrimerCircuit::<Fr> {
            a: None,
            b: None,
            c: None,
        };

        // Generate random parameters for the blank circuit
        let params = generate_random_parameters::<Bn254, _, _>(blank_circuit, &mut rng)
            .expect("Parameter generation failed");

        // Prepare the verifying key from the generated ProvingKey
        let pvk = prepare_verifying_key(&params.vk);

        // Save the ProvingKey and PreparedVerifyingKey to disk
        save_proving_key(&params).expect("failed to save pk");
        save_verifying_key(&pvk).expect("failed to save vk");

        (params, pvk)
    }
}

fn fq_to_u256(value: &Fq) -> U256 {
    // `into_repr()` returns a `BigInteger256` in arkworks v0.3
    let bi = value.into_repr();
    let bytes_be = bi.to_bytes_be(); // big-endian bytes
    U256::from_big_endian(&bytes_be)
}

/// A struct to hold all the parameters we need for our Solidity constructor.
pub struct VkParams {
    pub alpha_x: U256,
    pub alpha_y: U256,
    pub beta_x: [U256; 2],
    pub beta_y: [U256; 2],
    pub gamma_x: [U256; 2],
    pub gamma_y: [U256; 2],
    pub delta_x: [U256; 2],
    pub delta_y: [U256; 2],
    pub gamma_abc0_x: U256,
    pub gamma_abc0_y: U256,
    pub gamma_abc1_x: U256,
    pub gamma_abc1_y: U256,
}

/// Extracts the verification key parameters from an Arkworks `PreparedVerifyingKey<Bn254>`,
/// suitable for passing into the ZkVerifier.sol constructor (which expects 1 public input).
pub fn extract_vk_params(pvk: &PreparedVerifyingKey<Bn254>) -> VkParams {
    let vk = &pvk.vk;

    // alpha_g1
    let alpha_x = fq_to_u256(&vk.alpha_g1.x);
    let alpha_y = fq_to_u256(&vk.alpha_g1.y);

    // beta_g2 (x is Fq2 => x.c0, x.c1; same for y)
    let beta_x0 = fq_to_u256(&vk.beta_g2.x.c0);
    let beta_x1 = fq_to_u256(&vk.beta_g2.x.c1);
    let beta_y0 = fq_to_u256(&vk.beta_g2.y.c0);
    let beta_y1 = fq_to_u256(&vk.beta_g2.y.c1);

    // gamma_g2
    let gamma_x0 = fq_to_u256(&vk.gamma_g2.x.c0);
    let gamma_x1 = fq_to_u256(&vk.gamma_g2.x.c1);
    let gamma_y0 = fq_to_u256(&vk.gamma_g2.y.c0);
    let gamma_y1 = fq_to_u256(&vk.gamma_g2.y.c1);

    // delta_g2
    let delta_x0 = fq_to_u256(&vk.delta_g2.x.c0);
    let delta_x1 = fq_to_u256(&vk.delta_g2.x.c1);
    let delta_y0 = fq_to_u256(&vk.delta_g2.y.c0);
    let delta_y1 = fq_to_u256(&vk.delta_g2.y.c1);

    // gamma_abc_g1 is an array. For a single public input, we expect 2 points:
    //   gamma_abc[0] => constant term
    //   gamma_abc[1] => coefficient for the 1 input
    assert_eq!(vk.gamma_abc_g1.len(), 2, "Expected 2 gamma_abc points");

    let gamma_abc0 = &vk.gamma_abc_g1[0];
    let gamma_abc1 = &vk.gamma_abc_g1[1];
    let gamma_abc0_x = fq_to_u256(&gamma_abc0.x);
    let gamma_abc0_y = fq_to_u256(&gamma_abc0.y);
    let gamma_abc1_x = fq_to_u256(&gamma_abc1.x);
    let gamma_abc1_y = fq_to_u256(&gamma_abc1.y);

    VkParams {
        alpha_x,
        alpha_y,
        beta_x: [beta_x0, beta_x1],
        beta_y: [beta_y0, beta_y1],
        gamma_x: [gamma_x0, gamma_x1],
        gamma_y: [gamma_y0, gamma_y1],
        delta_x: [delta_x0, delta_x1],
        delta_y: [delta_y0, delta_y1],
        gamma_abc0_x,
        gamma_abc0_y,
        gamma_abc1_x,
        gamma_abc1_y,
    }
}

/// Example function that returns the parameters for the verifying key,
/// generating or loading them from disk first.
pub fn get_verifier_params() -> VkParams {
    // Suppose you already have your get_or_create_pvks() method:
    let (_pk, pvk) = get_or_create_pvks();

    // Then extract
    extract_vk_params(&pvk)
}
