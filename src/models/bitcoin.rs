use serde::{Deserialize, Serialize};

// Transaction input
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TxInput {
    pub prev_out: Option<PrevOut>,
    #[serde(default)]
    pub script: String,
}

// Previous output
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PrevOut {
    pub addr: Option<String>,
    pub value: u64,
}

// Transaction output
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TxOutput {
    pub addr: Option<String>,
    pub value: u64,
    #[serde(default)]
    pub script: String,
}

// Transaction
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Transaction {
    pub hash: String,
    #[serde(default)]
    pub time: u64,
    #[serde(default)]
    pub size: u32,
    #[serde(default)]
    pub inputs: Vec<TxInput>,
    #[serde(default)]
    pub out: Vec<TxOutput>,
}

// Raw address info from blockchain.info API
#[derive(Debug, Deserialize)]
pub struct AddressInfo {
    pub address: String,
    pub n_tx: u32,
    pub total_received: u64,
    pub total_sent: u64,
    pub final_balance: u64,
    #[serde(default)]
    pub txs: Vec<Transaction>,
}

// Pagination metadata
#[derive(Debug, Serialize)]
pub struct Pagination {
    pub total_transactions: u32,
    pub per_page: u32,
    pub current_page: u32,
    pub total_pages: u32,
}

// Address response with pagination
#[derive(Debug, Serialize)]
pub struct AddressWithPagination {
    pub address: String,
    pub total_received: u64,
    pub total_sent: u64,
    pub final_balance: u64,
    pub transactions: Vec<Transaction>,
    pub pagination: Pagination,
}

// Response wrapper for Bitcoin address queries
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum AddressResponse {
    Success(AddressWithPagination),
    Error { error: String },
}
