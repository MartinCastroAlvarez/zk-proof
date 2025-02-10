use ark_bn254::{Bn254, Fr};
use ark_groth16::{create_random_proof, verify_proof, PreparedVerifyingKey, Proof, ProvingKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use rand::thread_rng;

use crate::circuit::PrimerCircuit;

/// Generates a proof for the given inputs and parameters.
///
/// # Example
///
/// ```
/// use ark_bn254::Fr;
/// use my_crate::proof::generate_proof; // Adjust the path as necessary
/// use my_crate::proof::generate_initial_params; // Adjust the path as necessary
///
/// let params = generate_initial_params();
/// let a = Fr::from(3u32);
/// let b = Fr::from(4u32);
/// let (proof_base64, public_input) = generate_proof(a, b, &params);
///
/// assert!(!proof_base64.is_empty());
/// ```
pub fn generate_proof(a: Fr, b: Fr, params: &ProvingKey<Bn254>) -> (String, Fr) {
    let mut rng = thread_rng();
    let c = a + b;

    let circuit_with_witness = PrimerCircuit::<Fr> {
        a: Some(a),
        b: Some(b),
        c: Some(c),
    };

    let proof = create_random_proof(circuit_with_witness, params, &mut rng)
        .expect("Proof generation failed");

    let mut proof_bytes = Vec::new();
    proof
        .serialize(&mut proof_bytes)
        .expect("Failed to serialize proof");
    let proof_base64 = BASE64.encode(proof_bytes);

    (proof_base64, c)
}

/// Verifies a proof given its base64 string representation and public input.
///
/// # Example
///
/// ```
/// use ark_bn254::Fr;
/// use my_crate::proof::{generate_proof, verify_proof_string, generate_initial_params}; // Adjust the path as necessary
/// use ark_groth16::prepare_verifying_key;
///
/// let params = generate_initial_params();
/// let pvk = prepare_verifying_key(&params.vk);
///
/// let a = Fr::from(3u32);
/// let b = Fr::from(4u32);
/// let (proof_base64, public_input) = generate_proof(a, b, &params);
///
/// let is_valid = verify_proof_string(&proof_base64, public_input, &pvk);
/// assert!(is_valid);
/// ```
pub fn verify_proof_string(
    proof_base64: &str,
    public_input: Fr,
    pvk: &PreparedVerifyingKey<Bn254>,
) -> bool {
    let proof_bytes = BASE64.decode(proof_base64).expect("Failed to decode proof");
    let proof = Proof::deserialize(&proof_bytes[..]).expect("Failed to deserialize proof");

    verify_proof(pvk, &proof, &[public_input]).unwrap_or(false)
}
