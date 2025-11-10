mod api;
mod models;

use axum::{response::Json, routing::get, Router};

use api::{get_address_handler, get_evm_address_handler};

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Build the router with routes
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/btc/:address", get(get_address_handler))
        .route(
            "/evm/:chain_identifier/:address",
            get(get_evm_address_handler),
        );

    // Define the address to listen on
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind to port 8080");

    println!("Server running on http://0.0.0.0:8080");
    println!("\n=== Endpoints ===");
    println!("\nBitcoin:");
    println!("  GET /btc/<bitcoin_address>?page=<page>&limit=<limit>");
    println!("\nEVM Chains:");
    println!("  GET /evm/<chain_identifier>/<address>?page=<page>&limit=<limit>");
    println!("\n=== Examples ===");
    println!("\nBitcoin:");
    println!("  curl http://localhost:8080/btc/1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa");
    println!("  curl http://localhost:8080/btc/1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa?page=1&limit=10");
    println!("\nEthereum:");
    println!("  curl http://localhost:8080/evm/ethereum/0xYourAddress");
    println!("  curl http://localhost:8080/evm/0xYourAddress?page=1&limit=10");
    println!("\nPolygon:");
    println!("  curl http://localhost:8080/evm/polygon/0xYourAddress?page=2&limit=20");

    // Start the server
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

// Root route handler
async fn root_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": "Blockchain Address Info API",
        "endpoints": {
            "bitcoin": {
                "path": "GET /btc/{bitcoin_address}",
                "query_params": {
                    "page": "Page number (default: 1)",
                    "limit": "Transactions per page (default: 10, max: 50)"
                },
                "examples": [
                    "GET /btc/1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
                    "GET /btc/1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa?page=1&limit=10"
                ],
                "description": "Get Bitcoin address info with paginated transactions. Returns total transaction count and total pages."
            },
            "evm": {
                "path": "GET /evm/{chain_identifier}/{address}",
                "query_params": {
                    "page": "Page number (default: 1)",
                    "limit": "Transactions per page (default: 10, max: 50)"
                },
                "examples": [
                    "GET /evm/ethereum/0xYourAddress",
                    "GET /evm/1/0xYourAddress?page=1&limit=10",
                    "GET /evm/polygon/0xYourAddress?page=2&limit=20"
                ],
                "description": "Get EVM address info with paginated transactions (fast, no total count)",
                "supported_chains": {
                    "ethereum": "1",
                    "sepolia": "11155111",
                    "polygon": "137",
                    "polygon-amoy": "80002",
                    "bsc": "56",
                    "bsc-testnet": "97",
                    "arbitrum": "42161",
                    "arbitrum-sepolia": "421614",
                    "optimism": "10",
                    "optimism-sepolia": "11155420",
                    "base": "8453",
                    "base-sepolia": "84532"
                }
            }
        }
    }))
}
