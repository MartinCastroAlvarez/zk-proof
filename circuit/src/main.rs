use env_logger;
use log::{error, info};
use std::sync::Arc;
use warp::Filter;

mod auth;
mod circuit;
mod contracts;
mod files;
mod proof;
mod routes;
mod security;
mod types;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Initialize the logger
    env_logger::init();

    info!("Starting the Warp server...");

    // Use the get_or_create_vk function to ensure consistent VK across restarts
    let (proving_key, prepared_vk) = security::get_or_create_pvks();
    let proving_params = Arc::new(proving_key);
    let verifying_key = Arc::new(prepared_vk);

    // Print the verification key to stdout
    println!(
        "Verification Key (Base64): {}",
        security::to_base64(&verifying_key)
    );

    // Setup CORS.
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST"])
        .allow_headers(vec!["Content-Type"]);

    // Setup logging.
    let api = routes::proof_routes(proving_params, verifying_key)
        .with(cors)
        .with(warp::log::custom(|info| {
            let method = info.method();
            let path = info.path();
            let status = info.status();
            let elapsed = info.elapsed();
            let ip = info
                .remote_addr()
                .map(|addr| addr.to_string())
                .unwrap_or_else(|| "Unknown".into());

            if info.status().is_success() {
                info!(
                    "Request: {} {} from {} took {:?}",
                    method, path, ip, elapsed
                );
            } else {
                error!(
                    "Request: {} {} from {} failed with status {:?} and took {:?}",
                    method, path, ip, status, elapsed
                );
            }
        }));

    println!("Server starting on http://localhost:3030");
    warp::serve(api).run(([0, 0, 0, 0], 3030)).await;

    info!("Warp server has stopped.");
}
