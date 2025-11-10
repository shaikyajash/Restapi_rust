use serde::{Deserialize, Serialize};

// Simple pagination without total count
#[derive(Debug, Serialize)]
pub struct EvmPagination {
    pub per_page: u32,
    pub current_page: u32,
}

// Single EVM transaction
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EvmTransaction {
    pub block_number: String,
    pub time_stamp: String,
    pub hash: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub gas: String,
    pub gas_price: String,
    pub gas_used: String,
    pub is_error: String,
    #[serde(default)]
    pub txreceipt_status: String,
    #[serde(default)]
    pub input: String,
    #[serde(default)]
    pub contract_address: String,
}

// Etherscan API response for transactions
#[derive(Debug, Deserialize)]
pub struct EtherscanTxResponse {
    pub status: String,
    pub message: String,
    pub result: Vec<EvmTransaction>,
}

// Etherscan API response for balance
#[derive(Debug, Deserialize)]
pub struct EtherscanBalanceResponse {
    pub status: String,
    pub message: String,
    pub result: String,
}

// Address info with paginated transactions
#[derive(Debug, Serialize)]
pub struct EvmAddressWithTransactions {
    pub address: String,
    pub chain_id: u32,
    pub chain_name: String,
    pub balance: String,
    pub transactions: Vec<EvmTransaction>,
    pub pagination: EvmPagination,
}

// Response wrapper for EVM address queries
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum EvmAddressResponse {
    Success(EvmAddressWithTransactions),
    Error { error: String },
}
