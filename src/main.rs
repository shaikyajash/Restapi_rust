mod api;
mod models;

use axum::{
    response::Json,
    routing::get,
    Router,
};

use api::get_address_handler;

#[tokio::main]
async fn main() {
    // Build the router with routes
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/address/:address", get(get_address_handler));

    // Define the address to listen on
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind to port 8080");

    println!("Server running on http://0.0.0.0:8080");
    println!("Endpoints:");
    println!("  - GET /address/<bitcoin_address>?page=<page_number>&limit=<per_page>");
    println!("\nExamples:");
    println!("  - curl http://localhost:8080/address/1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa");
    println!("  - curl http://localhost:8080/address/1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa?page=1&limit=10");
    println!("  - curl http://localhost:8080/address/1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa?page=2&limit=20");

    // Start the server
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

// Root route handler
async fn root_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": "Bitcoin Address Info API",
        "endpoints": {
            "bitcoin": {
                "path": "GET /address/{bitcoin_address}",
                "query_params": {
                    "page": "Page number (default: 1)",
                    "limit": "Transactions per page (default: 10, max: 50)"
                },
                "examples": [
                    "GET /address/1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
                    "GET /address/1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa?page=1&limit=10",
                    "GET /address/1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa?page=2&limit=20"
                ],
                "description": "Get transaction details, balance, and paginated transactions for any Bitcoin address"
            }
        }
    }))
}
