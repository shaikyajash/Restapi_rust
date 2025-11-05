# Blockchain API

A REST API server for querying Bitcoin blockchain data.

## Features

- **Bitcoin Address Info**: Get transaction details and balance for any Bitcoin address

## Setup

1. Clone the repository
2. Install Rust (if not already installed)

## Running

```bash
cargo run
```

The server will start on `http://0.0.0.0:8080`

## API Endpoints

### Root
```bash
GET /
```
Returns API information and available endpoints.

### Bitcoin Address Info
```bash
GET /address/{bitcoin_address}
```

Example:
```bash
curl http://localhost:8080/address/1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa
```

Response:
```json
{
  "address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",
  "n_tx": 100,
  "total_received": 1000000000,
  "total_sent": 500000000,
  "final_balance": 500000000
}
```



## Dependencies
## Dependencies

- `axum` - Web framework
- `reqwest` - HTTP client
- `serde` - Serialization
- `tokio` - Async runtime
- `anyhow` - Error handling
