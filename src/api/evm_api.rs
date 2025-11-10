use crate::models::{
    EtherscanBalanceResponse, EtherscanTxResponse, EvmAddressResponse, 
    EvmAddressWithTransactions, EvmPagination,
};
use axum::{extract::{Path, Query}, response::Json};
use reqwest::StatusCode;
use serde::Deserialize;
use std::env;

// Query parameters for pagination
#[derive(Debug, Deserialize)]
pub struct EvmPaginationQuery {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_page() -> u32 {
    1
}

fn default_limit() -> u32 {
    10
}

// Helper function to get chain name from ID
fn get_chain_name(chain_id: u32) -> String {
    match chain_id {
        1 => "Ethereum Mainnet".to_string(),
        11155111 => "Sepolia Testnet".to_string(),
        137 => "Polygon Mainnet".to_string(),
        80002 => "Polygon Amoy Testnet".to_string(),
        56 => "BSC Mainnet".to_string(),
        97 => "BSC Testnet".to_string(),
        42161 => "Arbitrum One".to_string(),
        421614 => "Arbitrum Sepolia".to_string(),
        10 => "Optimism Mainnet".to_string(),
        11155420 => "Optimism Sepolia".to_string(),
        8453 => "Base Mainnet".to_string(),
        84532 => "Base Sepolia".to_string(),
        _ => format!("Chain {}", chain_id),
    }
}

// Parse chain identifier (name or ID)
fn parse_chain_identifier(identifier: &str) -> Option<u32> {
    // Try parsing as number first
    if let Ok(id) = identifier.parse::<u32>() {
        return Some(id);
    }
    
    // Try parsing as chain name
    match identifier.to_lowercase().as_str() {
        "ethereum" | "eth" => Some(1),
        "sepolia" => Some(11155111),
        "polygon" | "matic" => Some(137),
        "polygon-amoy" | "amoy" => Some(80002),
        "bsc" | "binance" => Some(56),
        "bsc-testnet" => Some(97),
        "arbitrum" | "arb" => Some(42161),
        "arbitrum-sepolia" => Some(421614),
        "optimism" | "opt" => Some(10),
        "optimism-sepolia" => Some(11155420),
        "base" => Some(8453),
        "base-sepolia" => Some(84532),
        _ => None,
    }
}

// Fetch address info with transactions from Etherscan (fast, no total count)
pub async fn fetch_evm_address_info(
    address: &str,
    chain_id: u32,
    page: u32,
    limit: u32,
) -> Result<EvmAddressWithTransactions, anyhow::Error> {
    let api_key = env::var("ETHERSCAN_API_KEY")
        .unwrap_or_else(|_| "YourApiKeyToken".to_string());
    
    // Limit to max 50 per page for reasonable response times
    let limit = limit.min(50);
    
    // Etherscan uses page and offset (offset = limit)
    let tx_url = format!(
        "https://api.etherscan.io/v2/api?apikey={}&chainid={}&module=account&action=txlist&address={}&startblock=0&endblock=99999999&page={}&offset={}&sort=desc",
        api_key, chain_id, address, page, limit
    );
    
    // Fetch balance
    let balance_url = format!(
        "https://api.etherscan.io/v2/api?apikey={}&chainid={}&module=account&action=balance&address={}&tag=latest",
        api_key, chain_id, address
    );
    
    let client = reqwest::Client::new();
    
    // Fetch both in parallel
    let (tx_response, balance_response) = tokio::join!(
        client.get(&tx_url).send(),
        client.get(&balance_url).send()
    );
    
    let tx_response = tx_response?;
    let balance_response = balance_response?;
    
    if tx_response.status() != StatusCode::OK {
        return Err(anyhow::anyhow!(
            "Failed to fetch transactions: HTTP {}",
            tx_response.status()
        ));
    }
    
    if balance_response.status() != StatusCode::OK {
        return Err(anyhow::anyhow!(
            "Failed to fetch balance: HTTP {}",
            balance_response.status()
        ));
    }
    
    let tx_data = tx_response.json::<EtherscanTxResponse>().await?;
    let balance_data = balance_response.json::<EtherscanBalanceResponse>().await?;
    
    // Check for errors
    if tx_data.status != "1" && !tx_data.message.contains("No transactions found") {
        return Err(anyhow::anyhow!("Etherscan API error: {}", tx_data.message));
    }
    
    if balance_data.status != "1" {
        return Err(anyhow::anyhow!("Etherscan API error: {}", balance_data.message));
    }
    
    Ok(EvmAddressWithTransactions {
        address: address.to_string(),
        chain_id,
        chain_name: get_chain_name(chain_id),
        balance: balance_data.result,
        transactions: tx_data.result,
        pagination: EvmPagination {
            per_page: limit,
            current_page: page,
        },
    })
}

// Handler for fetching EVM address info with pagination
pub async fn get_evm_address_handler(
    Path((chain_identifier, address)): Path<(String, String)>,
    Query(params): Query<EvmPaginationQuery>,
) -> Json<EvmAddressResponse> {
    // Parse chain identifier (can be name like "ethereum" or ID like "1")
    let chain_id = match parse_chain_identifier(&chain_identifier) {
        Some(id) => id,
        None => {
            return Json(EvmAddressResponse::Error {
                error: format!(
                    "Invalid chain identifier: '{}'. Use chain name (e.g., 'ethereum', 'polygon') or chain ID (e.g., '1', '137')",
                    chain_identifier
                ),
            });
        }
    };
    
    // Ensure page is at least 1
    let page = params.page.max(1);
    let limit = params.limit.max(1).min(50);
    
    match fetch_evm_address_info(&address, chain_id, page, limit).await {
        Ok(info) => Json(EvmAddressResponse::Success(info)),
        Err(e) => Json(EvmAddressResponse::Error {
            error: format!("Failed to fetch address info: {}", e),
        }),
    }
}
