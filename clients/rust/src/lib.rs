//! Jupiter API client.
//!
//! Prefer [`JupiterClient`] over calling generated API modules directly.

#![allow(clippy::too_many_arguments)]

mod client;
mod config;
mod generated_impls;

pub use client::{
    JupiterClient, Lend, LendApis, LendBorrow, Portfolio, Prediction, PriceApis, PriceV2, PriceV3,
    Recurring, SendApi, Studio, SwapApis, SwapV1, SwapV2, TokensApis, TokensV1, TokensV2,
    TokensV2Verification, Transaction, TriggerApis, TriggerV1, TriggerV2, Ultra,
};
pub use config::{JupiterClientBaseUrls, JupiterClientOptions, JupiterClientService};

/// Generated APIs and models. Prefer [`JupiterClient`] for product-nested access.
pub use jup_ag_api_generated::{self as generated, apis::Error, models};
