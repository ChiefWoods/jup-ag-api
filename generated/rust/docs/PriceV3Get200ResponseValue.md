# PriceV3Get200ResponseValue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | When the token was minted or created (fixed date for legacy tokens like SOL/USDC that predate the API) | 
**liquidity** | **f64** | Total liquidity in USD across all pools | 
**usd_price** | **f64** | Current USD price | 
**block_id** | Option<**i32**> | Solana block ID when price was computed | 
**decimals** | **i32** | Token decimal places | 
**price_change24h** | Option<**f64**> | 24-hour price change as a percentage | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


