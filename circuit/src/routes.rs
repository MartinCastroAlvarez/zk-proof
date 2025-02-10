use ark_bn254::{Bn254, Fr};
use ark_groth16::{PreparedVerifyingKey, ProvingKey};
use log::error;
use serde_json::json;
use std::str::FromStr;
use std::sync::Arc;
use warp::reject::Reject;
use warp::{Filter, Rejection, Reply};

use crate::auth::{get_public_address, set_credentials};
use crate::proof;
use crate::security;
use crate::types::{AuthRequest, ProofRequest, ProofResponse, VerifyRequest, VerifyResponse};
use crate::contracts::save_contract_address;

#[derive(Debug)]
struct InvalidInputError;

impl Reject for InvalidInputError {}

#[derive(Debug)]
struct SoftError {
    message: String,
}

impl Reject for SoftError {}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    if let Some(soft_error) = err.find::<SoftError>() {
        let json = warp::reply::json(&serde_json::json!({ "error": soft_error.message }));
        let response = warp::reply::with_status(json, warp::http::StatusCode::OK);
        Ok(response)
    } else {
        // Handle other kinds of errors or pass them through
        let code = if err.is_not_found() {
            warp::http::StatusCode::NOT_FOUND
        } else {
            warp::http::StatusCode::INTERNAL_SERVER_ERROR
        };
        let json = warp::reply::json(&serde_json::json!({ "error": "Unhandled error" }));
        let response = warp::reply::with_status(json, code);
        Ok(response)
    }
}

/// Sets up the proof routes for the web server.
///
/// # Example
///
/// ```
/// use std::sync::Arc;
/// use warp::Filter;
/// use my_crate::routes::proof_routes; // Adjust the path as necessary
/// use my_crate::vk::get_or_create_vk; // Adjust the path as necessary
/// use my_crate::proof::generate_initial_params; // Adjust the path as necessary
///
/// let proving_params = Arc::new(generate_initial_params());
/// let verifying_key = Arc::new(get_or_create_vk());
///
/// let routes = proof_routes(proving_params, verifying_key);
/// warp::test::request()
///     .method("POST")
///     .path("/generate")
///     .json(&serde_json::json!({"a": "3", "b": "4"}))
///     .reply(&routes)
///     .await;
/// ```
pub fn proof_routes(
    proving_params: Arc<ProvingKey<Bn254>>,
    verifying_key: Arc<PreparedVerifyingKey<Bn254>>,
) -> impl Filter<Extract = (impl Reply,), Error = std::convert::Infallible> + Clone {
    let generate = generate_proof_route(proving_params);
    let verify = verify_proof_route(verifying_key.clone());
    let export_vk = export_vk_route(verifying_key.clone());
    let health = health_route();
    let auth = auth_route();
    let save_address = save_contract_address_route();

    generate
        .or(verify)
        .or(export_vk)
        .or(health)
        .or(auth)
        .or(save_address)
        .recover(handle_rejection)
}

/// Creates the route for generating proofs.
///
/// # Example
///
/// ```
/// use std::sync::Arc;
/// use warp::Filter;
/// use my_crate::routes::generate_proof_route; // Adjust the path as necessary
/// use my_crate::proof::generate_initial_params; // Adjust the path as necessary
///
/// let proving_params = Arc::new(generate_initial_params());
/// let route = generate_proof_route(proving_params);
///
/// warp::test::request()
///     .method("POST")
///     .path("/generate")
///     .json(&serde_json::json!({"a": "3", "b": "4"}))
///     .reply(&route)
///     .await;
/// ```
fn generate_proof_route(
    proving_params: Arc<ProvingKey<Bn254>>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("generate")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || Arc::clone(&proving_params)))
        .and_then(handle_generate_proof)
}

/// Handles the proof generation logic.
///
/// # Example
///
/// ```
/// use std::sync::Arc;
/// use my_crate::routes::handle_generate_proof; // Adjust the path as necessary
/// use my_crate::proof::generate_initial_params; // Adjust the path as necessary
/// use my_crate::types::ProofRequest; // Adjust the path as necessary
///
/// let proving_params = Arc::new(generate_initial_params());
/// let req = ProofRequest { a: "3".to_string(), b: "4".to_string() };
///
/// let response = handle_generate_proof(req, proving_params).await;
/// assert!(response.is_ok());
/// ```
async fn handle_generate_proof(
    req: ProofRequest,
    params: Arc<ProvingKey<Bn254>>,
) -> Result<impl Reply, Rejection> {
    let a = Fr::from_str(&req.a).map_err(|_| warp::reject::custom(InvalidInputError))?;
    let b = Fr::from_str(&req.b).map_err(|_| warp::reject::custom(InvalidInputError))?;

    let (proof, public_input) = proof::generate_proof(a, b, &params);

    let response = ProofResponse {
        proof,
        public_input: public_input.to_string(),
    };

    Ok(warp::reply::json(&response))
}

/// Creates the route for verifying proofs.
///
/// # Example
///
/// ```
/// use std::sync::Arc;
/// use warp::Filter;
/// use my_crate::routes::verify_proof_route; // Adjust the path as necessary
/// use my_crate::vk::get_or_create_vk; // Adjust the path as necessary
///
/// let verifying_key = Arc::new(get_or_create_vk());
/// let route = verify_proof_route(verifying_key);
///
/// warp::test::request()
///     .method("POST")
///     .path("/verify")
///     .json(&serde_json::json!({"proof": "some_proof", "public_input": "7"}))
///     .reply(&route)
///     .await;
/// ```
fn verify_proof_route(
    verifying_key: Arc<PreparedVerifyingKey<Bn254>>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("verify")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || Arc::clone(&verifying_key)))
        .and_then(handle_verify_proof)
}

/// Handles the proof verification logic.
///
/// # Example
///
/// ```
/// use std::sync::Arc;
/// use my_crate::routes::handle_verify_proof; // Adjust the path as necessary
/// use my_crate::vk::get_or_create_vk; // Adjust the path as necessary
/// use my_crate::types::VerifyRequest; // Adjust the path as necessary
///
/// let verifying_key = Arc::new(get_or_create_vk());
/// let req = VerifyRequest { proof: "some_proof".to_string(), public_input: "7".to_string() };
///
/// let response = handle_verify_proof(req, verifying_key).await;
/// assert!(response.is_ok());
/// ```
async fn handle_verify_proof(
    req: VerifyRequest,
    vk: Arc<PreparedVerifyingKey<Bn254>>,
) -> Result<impl Reply, Rejection> {
    let public_input =
        Fr::from_str(&req.public_input).map_err(|_| warp::reject::custom(InvalidInputError))?;
    let is_valid = proof::verify_proof_string(&req.proof, public_input, &vk);

    let response = VerifyResponse { is_valid };
    Ok(warp::reply::json(&response))
}

/// Creates the route for exporting the verification key.
///
/// # Example
///
/// ```
/// use std::sync::Arc;
/// use warp::Filter;
/// use my_crate::routes::export_vk_route; // Adjust the path as necessary
/// use my_crate::vk::get_or_create_vk; // Adjust the path as necessary
///
/// let verifying_key = Arc::new(get_or_create_vk());
/// let route = export_vk_route(verifying_key);
///
/// warp::test::request()
///     .method("GET")
///     .path("/vk")
///     .reply(&route)
///     .await;
/// ```
fn export_vk_route(
    verifying_key: Arc<PreparedVerifyingKey<Bn254>>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("vk")
        .and(warp::get())
        .and(warp::any().map(move || Arc::clone(&verifying_key)))
        .and_then(handle_export_vk)
}

/// Handles the logic for exporting the verification key.
///
/// # Example
///
/// ```
/// use std::sync::Arc;
/// use my_crate::routes::handle_export_vk; // Adjust the path as necessary
/// use my_crate::vk::get_or_create_vk; // Adjust the path as necessary
///
/// let verifying_key = Arc::new(get_or_create_vk());
///
/// let response = handle_export_vk(verifying_key).await;
/// assert!(response.is_ok());
/// ```
async fn handle_export_vk(vk: Arc<PreparedVerifyingKey<Bn254>>) -> Result<impl Reply, Rejection> {
    let vk_base64 = security::to_base64(&vk);
    let response = serde_json::json!({ "vk": vk_base64 });
    Ok(warp::reply::json(&response))
}

/// Creates the health check route.
///
/// # Example
///
/// ```
/// use warp::Filter;
/// use my_crate::routes::health_route; // Adjust the path as necessary
///
/// let route = health_route();
///
/// warp::test::request()
///     .method("GET")
///     .path("/health")
///     .reply(&route)
///     .await;
/// ```
fn health_route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("health")
        .and(warp::get())
        .map(|| warp::reply::json(&"Ok"))
}

/// Creates the route for setting credentials.
///
/// # Example
///
/// ```
/// use warp::Filter;
/// let route = auth_route();
/// ```
fn auth_route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    // POST /auth: sets credentials
    let post_auth = warp::path("auth")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_set_credentials);

    // GET /auth: returns public address
    let get_auth = warp::path("auth")
        .and(warp::get())
        .and_then(handle_get_public_address);

    // Combine the two routes
    post_auth.or(get_auth)
}

/// Handles setting the credentials.
///
/// # Example
///
/// ```
/// use warp::Filter;
/// use crate::auth::{set_credentials, get_public_address};
///
/// let route = auth_route();
///
/// // Set credentials
/// let address = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string();
/// let secret_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string();
/// let response = warp::test::request()
///     .method("POST")
///     .path("/auth")
///     .json(&serde_json::json!({"address": address, "secret_key": secret_key}))
///     .reply(&route)
///     .await;
/// assert_eq!(response.status(), 200);
///
/// // Get public address
/// let response = warp::test::request()
///     .method("GET")
///     .path("/auth")
///     .reply(&route)
///     .await;
/// assert_eq!(response.status(), 200);
/// assert_eq!(response.body(), "\"0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266\"");
/// ```
async fn handle_set_credentials(req: AuthRequest) -> Result<impl Reply, Rejection> {
    set_credentials(req.address.clone(), req.secret_key)
        .map(|_| {
            // Return the address in the desired JSON format
            let response = json!({ "address": req.address });
            warp::reply::json(&response)
        })
        .map_err(|_| warp::reject::custom(InvalidInputError))
}

/// Handles getting the public address.
///
/// # Example
///
/// ```
/// use std::sync::Arc;
/// use my_crate::routes::handle_get_public_address; // Adjust the path as necessary
/// use my_crate::auth::get_public_address; // Adjust the path as necessary
///
/// let response = handle_get_public_address().await;
/// assert!(response.is_ok());
/// ```
async fn handle_get_public_address() -> Result<impl Reply, Rejection> {
    match get_public_address() {
        Some(address) => {
            // Return the address in the desired JSON format
            let response = json!({ "address": address });
            Ok(warp::reply::json(&response))
        }
        None => Err(warp::reject::not_found()),
    }
}

/// Creates the route for saving the contract address.
///
/// # Example
///
/// ```
/// use warp::Filter;
/// use my_crate::routes::save_contract_address_route; // Adjust the path as necessary
///
/// let route = save_contract_address_route();
///
/// let response = warp::test::request()
///     .method("POST")
///     .path("/contract")
///     .json(&serde_json::json!({"address": "0x1234567890abcdef1234567890abcdef12345678"}))
///     .reply(&route)
///     .await;
/// assert_eq!(response.status(), 200);
/// ```
fn save_contract_address_route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("contract")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_save_contract_address)
}

/// Handles saving the contract address.
///
/// # Example
///
/// ```
/// use my_crate::routes::handle_save_contract_address; // Adjust the path as necessary
/// use my_crate::contracts::save_contract_address; // Adjust the path as necessary
///
/// let req = json!({"address": "0x1234567890abcdef1234567890abcdef12345678"});
/// let response = handle_save_contract_address(req).await;
/// assert!(response.is_ok());
/// ```
async fn handle_save_contract_address(req: serde_json::Value) -> Result<impl Reply, Rejection> {
    let address = req.get("address")
        .and_then(|v| v.as_str())
        .ok_or_else(|| warp::reject::custom(InvalidInputError))?;

    save_contract_address(address).map_err(|_| warp::reject::custom(InvalidInputError))?;
    Ok(warp::reply::json(&json!({ "status": "success" })))
}
