use crate::models::{
    EtherscanBalanceResponse, EtherscanTxResponse, EvmAddressResponse, EvmAddressWithTransactions,
    EvmPagination,
};
use axum::{
    extract::{Path, Query},
    response::Json,
};
use reqwest::StatusCode;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;

// Query parameters
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

// Chain configuration: name -> chain_id
fn get_chain_id(chain_name: &str) -> Option<u32> {
    let chains = HashMap::from([
        ("ethereum", 1),
        ("eth", 1),
        ("sepolia", 11155111),
        ("polygon", 137),
        ("matic", 137),
        ("amoy", 80002),
        ("bsc", 56),
        ("binance", 56),
        ("bsc-testnet", 97),
        ("arbitrum", 42161),
        ("arb", 42161),
        ("optimism", 10),
        ("opt", 10),
        ("base", 8453),
    ]);

    chains.get(chain_name.to_lowercase().as_str()).copied()
}

// Get chain name from chain_id
fn get_chain_name(chain_id: u32) -> String {
    match chain_id {
        1 => "Ethereum Mainnet".to_string(),
        11155111 => "Sepolia Testnet".to_string(),
        137 => "Polygon Mainnet".to_string(),
        80002 => "Polygon Amoy Testnet".to_string(),
        56 => "BSC Mainnet".to_string(),
        97 => "BSC Testnet".to_string(),
        42161 => "Arbitrum One".to_string(),
        10 => "Optimism Mainnet".to_string(),
        8453 => "Base Mainnet".to_string(),
        _ => format!("Chain {}", chain_id),
    }
}

// Fetch address info with transactions
pub async fn fetch_evm_address_info(
    address: &str,
    chain_id: u32,
    page: u32,
    limit: u32,
) -> Result<EvmAddressWithTransactions, anyhow::Error> {
    let api_key = env::var("ETHERSCAN_API_KEY").unwrap_or_else(|_| "YourApiKeyToken".to_string());

    let tx_url = format!(
        "https://api.etherscan.io/v2/api?apikey={}&chainid={}&module=account&action=txlist&address={}&startblock=0&endblock=99999999&page={}&offset={}&sort=desc",
        api_key, chain_id, address, page, limit
    );

    let balance_url = format!(
        "https://api.etherscan.io/v2/api?apikey={}&chainid={}&module=account&action=balance&address={}&tag=latest",
        api_key, chain_id, address
    );

    let client = reqwest::Client::new();
    let (tx_response, balance_response) =
        tokio::join!(client.get(&tx_url).send(), client.get(&balance_url).send());

    let tx_response = tx_response?;
    let balance_response = balance_response?;

    match (tx_response.status(), balance_response.status()) {
        (StatusCode::OK, StatusCode::OK) => {}
        (tx_status, bal_status) => {
            return Err(anyhow::anyhow!(
                "HTTP error - tx: {}, balance: {}",
                tx_status,
                bal_status
            ));
        }
    }

    let tx_data = tx_response.json::<EtherscanTxResponse>().await?;
    let balance_data = balance_response.json::<EtherscanBalanceResponse>().await?;

    match (tx_data.status.as_str(), balance_data.status.as_str()) {
        ("1", "1") => {}
        (tx_s, bal_s) => {
            return Err(anyhow::anyhow!(
                "API error - tx: {}, balance: {}",
                tx_s,
                bal_s
            ));
        }
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

// Handler for fetching EVM address info
pub async fn get_evm_address_handler(
    Path((chain_name, address)): Path<(String, String)>,
    Query(params): Query<EvmPaginationQuery>,
) -> Json<EvmAddressResponse> {
    let chain_id = match get_chain_id(&chain_name) {
        Some(id) => id,
        None => {
            return Json(EvmAddressResponse::Error {
                error: format!(
                    "Invalid chain: '{}'. Use: ethereum, polygon, bsc, arbitrum, optimism, base",
                    chain_name
                ),
            });
        }
    };

    let page = params.page.max(1);
    let limit = params.limit.max(1).min(50);

    match fetch_evm_address_info(&address, chain_id, page, limit).await {
        Ok(info) => Json(EvmAddressResponse::Success(info)),
        Err(e) => Json(EvmAddressResponse::Error {
            error: format!("Failed to fetch address info: {}", e),
        }),
    }
}
