mod worker;

use warp::Filter;

#[tokio::main]
async fn main() {
    // Health check route.
    let health = warp::path("health").map(|| warp::reply::html("OK"));

    // Fibonacci route: POST /fib/<a>
    let phi_route = warp::path!("fib" / u64).and(warp::post()).map(|a: u64| {
        // Wrap both success and error in a reply with a status.
        match worker::phi(a) {
            Ok(result) => {
                warp::reply::with_status(warp::reply::json(&result), warp::http::StatusCode::OK)
            }
            Err(e) => warp::reply::with_status(
                warp::reply::json(&e),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ),
        }
    });

    // Combine the routes.
    let routes = health.or(phi_route);

    // Start the server on port 3030.
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
