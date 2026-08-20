# PredictionPosition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pubkey** | **String** | Position account public key | 
**owner** | **String** | Position owner public key | 
**owner_pubkey** | **String** | Position owner public key (alias of owner, prefer ownerPubkey going forward) | 
**market** | **String** | Deterministic market PDA derived from marketId | 
**market_id** | **String** | External market identifier (ticker) | 
**market_id_hash** | **String** | Hashed market identifier used for on-chain PDAs | 
**is_yes** | **bool** | True if this is a YES position | 
**contracts** | **String** | Legacy whole-contract quantity held. Use contractsMicro or contractsDecimal for the exact fractional quantity. | 
**contracts_micro** | Option<**String**> | Contracts held in micro-contract units (u64 as string, 1000000 = 1 contract) | [optional]
**contracts_decimal** | Option<**String**> | Contracts held as a decimal string | [optional]
**source** | Option<**String**> | Origin of the position (e.g. \"program\") | [optional]
**max_slippage_bps** | Option<**i32**> | Maximum slippage recorded for the position, in basis points | [optional]
**total_cost_usd** | **String** | Total cost basis in micro USD (u128 as string) | 
**size_usd** | **String** | Alias of total cost basis in micro USD (u128 as string) | 
**value_usd** | Option<**String**> | Mark-to-market value in micro USD (u128 as string); null once market is closed | 
**avg_price_usd** | **String** | Average entry price per contract in micro USD (u64 as string) | 
**mark_price_usd** | Option<**String**> | Current mark price per contract in micro USD (u64 as string); null once market is closed | 
**sell_price_usd** | Option<**String**> | Current best exit price for this side (micro USD string), or null when unavailable | 
**pnl_usd** | Option<**String**> | Unrealized PnL in micro USD (i128 as string); null once market is closed | 
**pnl_usd_percent** | Option<**f64**> | Unrealized PnL percentage relative to size; null once market is closed | 
**pnl_usd_after_fees** | Option<**String**> | Unrealized PnL after fees in micro USD (i128 as string); null once market is closed | 
**pnl_usd_after_fees_percent** | Option<**f64**> | Unrealized PnL after fees percentage relative to size; null once market is closed | 
**open_orders** | **i32** | Number of open orders | 
**fees_paid_usd** | **String** | Total fees paid in micro USD (u64 as string) | 
**realized_pnl_usd** | **f64** | Realized PnL in micro USD (i64 as number) | 
**claimed** | **bool** | Whether payout has been claimed | 
**claimed_usd** | **String** | Amount claimed in micro USD (u64 as string) | 
**opened_at** | **i32** | Unix timestamp when the position was opened | 
**updated_at** | **i32** | Unix timestamp of the last position update | 
**claimable_at** | Option<**i32**> | Unix timestamp (seconds) when the position becomes claimable; null when unavailable | 
**payout_usd** | **String** | Notional payout if position wins (contracts * $1) | 
**bump** | **i32** |  | 
**event_id** | **String** | External event identifier if available | 
**event_metadata** | [**models::PredictionEventMetadata**](PredictionEventMetadata.md) |  | 
**market_metadata** | [**models::PredictionOrderMarketMetadata**](PredictionOrderMarketMetadata.md) |  | 
**settlement_date** | Option<**i32**> | Unix timestamp (seconds) when the market settles; null when unavailable | 
**claimable** | **bool** | True when the position qualifies for payout claim | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


