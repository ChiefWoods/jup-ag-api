//! Rust client for Jupiter APIs generated from the repository's OpenAPI specs.

/// Generated client and types for endpoints hosted at `https://api.jup.ag`.
#[path = "../generated/jupiter/mod.rs"]
pub mod jupiter;

/// Generated client and types for transaction submission at `https://tx.jup.ag`.
#[path = "../generated/tx/mod.rs"]
pub mod tx;

/// Generated client and types for Perps endpoints hosted at `https://perps-api.jup.ag/v1`.
#[path = "../generated/perps/mod.rs"]
pub mod perps;

/// Generated client for endpoints hosted at `https://api.jup.ag`.
pub type JupiterApi = jupiter::HttpClient;

/// Generated client for transaction submission at `https://tx.jup.ag`.
pub type JupiterTxApi = tx::HttpClient;

/// Generated client for unauthenticated Perps endpoints hosted at `https://perps-api.jup.ag/v1`.
pub type JupiterPerpsApi = perps::HttpClient;

/// Creates a Jupiter API client configured with the required `x-api-key` header.
pub fn create_jupiter_api(api_key: impl Into<String>) -> JupiterApi {
    JupiterApi::new().with_api_key(api_key)
}

/// Creates a transaction-submission client configured with the required `x-api-key` header.
pub fn create_jupiter_tx_api(api_key: impl Into<String>) -> JupiterTxApi {
    JupiterTxApi::new().with_api_key(api_key)
}

/// Creates an unauthenticated Jupiter Perps API client.
pub fn create_jupiter_perps_api() -> JupiterPerpsApi {
    JupiterPerpsApi::new()
}
