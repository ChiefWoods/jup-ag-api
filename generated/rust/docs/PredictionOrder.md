# PredictionOrder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pubkey** | **String** | Order account public key | 
**owner** | **String** | Order owner public key | 
**owner_pubkey** | **String** | Order owner public key (alias of owner, use ownerPubkey when available) | 
**market** | **String** | Associated market public key | 
**market_id** | **String** | External market identifier used to derive the PDA | 
**market_id_hash** | **String** | Hashed market identifier used for on-chain PDAs | 
**event_id** | **String** | External event identifier | 
**position** | **String** | Associated position public key | 
**status** | **Status** | Current order status (enum: pending, filled, failed) | 
**is_yes** | **bool** | True when order is for the YES side | 
**is_buy** | **bool** | True when order is a buy order | 
**created_at** | **i32** | Unix timestamp (seconds) when the order was created | 
**updated_at** | **i32** | Unix timestamp (seconds) when the order last changed on-chain | 
**contracts** | **String** | Number of contracts (u64 as string) | 
**max_fill_price_usd** | **String** | Maximum fill price in micro USD (u64 as string) | 
**max_buy_price_usd** | Option<**String**> | Buyer-specified max fill price (micro USD) | 
**min_sell_price_usd** | Option<**String**> | Seller-specified min fill price (micro USD) | 
**filled_at** | **i32** | Unix timestamp (seconds) when the order was filled (0 when pending) | 
**filled_contracts** | **String** | Number of filled contracts (u64 as string) | 
**avg_fill_price_usd** | **String** | Average fill price in micro USD (u64 as string) | 
**settled** | **bool** | Whether the order has been settled on-chain | 
**order_id** | **String** | External order identifier from the venue | 
**size_usd** | **String** | Order notional in micro USD (u128 as string) | 
**event_metadata** | [**models::PredictionEventMetadata**](PredictionEventMetadata.md) |  | 
**market_metadata** | [**models::PredictionOrderMarketMetadata**](PredictionOrderMarketMetadata.md) |  | 
**external_order_id** | **String** | Client-provided order identifier | 
**bump** | **i32** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


