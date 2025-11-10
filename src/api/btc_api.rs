use crate::models::{AddressInfo, AddressResponse, AddressWithPagination, Pagination};
use axum::{
    extract::{Path, Query},
    response::Json,
};
use reqwest::StatusCode;
use serde::Deserialize;

// Query parameters for pagination
#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
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

pub async fn fetch_address_info(
    address: &str,
    offset: u32,
    limit: u32,
) -> Result<AddressInfo, anyhow::Error> {
    // Blockchain.info API has a max limit of 50
    let limit = limit.min(50);

    let url = format!(
        "https://blockchain.info/rawaddr/{}?offset={}&limit={}",
        address, offset, limit
    );

    let response = reqwest::get(&url).await?;

    if response.status() != StatusCode::OK {
        return Err(anyhow::anyhow!(
            "Failed to fetch address info: HTTP {}",
            response.status()
        ));
    }

    let address_info = response.json::<AddressInfo>().await?;

    Ok(address_info)
}

// Handler for fetching address info with pagination
pub async fn get_address_handler(
    Path(address): Path<String>,
    Query(params): Query<PaginationQuery>,
) -> Json<AddressResponse> {
    // Ensure page is at least 1
    let page = params.page.max(1);
    let limit = params.limit.max(1).min(50);

    // Calculate offset from page number (page 1 = offset 0)
    let offset = (page - 1) * limit;

    match fetch_address_info(&address, offset, limit).await {
        Ok(info) => {
            // Calculate total pages
            let total_pages = if info.n_tx == 0 {
                1
            } else {
                (info.n_tx + limit - 1) / limit // Ceiling division
            };

            Json(AddressResponse::Success(AddressWithPagination {
                address: info.address,
                total_received: info.total_received,
                total_sent: info.total_sent,
                final_balance: info.final_balance,
                transactions: info.txs,
                pagination: Pagination {
                    total_transactions: info.n_tx,
                    per_page: limit,
                    current_page: page,
                    total_pages,
                },
            }))
        }
        Err(e) => Json(AddressResponse::Error {
            error: format!("Failed to fetch address info: {}", e),
        }),
    }
}
