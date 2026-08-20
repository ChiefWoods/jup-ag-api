# TriggerV2OrdersDcaPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**deposit_request_id** | **String** | requestId from the deposit craft response | 
**deposit_signed_tx** | **String** | Base64-encoded signed deposit transaction | 
**user_pubkey** | **String** | Your wallet public key (must match the JWT) | 
**input_mint** | **String** | Mint of the token to sell | 
**output_mint** | **String** | Mint of the token to buy | 
**input_amount** | **String** | Total amount to DCA in smallest units. Dust from uneven division is added to the last round. | 
**order_count** | **f64** | Number of rounds to split the deposit into (minimum 2). No fixed upper limit, but each round must be worth at least 10 USD, so the practical maximum is floor(total deposit USD / 10). | 
**interval_seconds** | **f64** | Seconds between rounds (60 to 31,536,000, i.e. 1 minute to 1 year) | 
**order_type** | Option<**OrderType**> | time_based runs every interval; price_conditional only fills while the trigger mint price is within the band. (enum: time_based, price_conditional) | [optional][default to TimeBased]
**min_price_usd** | Option<**f64**> | Lower USD price bound. Price-conditional only. | [optional]
**max_price_usd** | Option<**f64**> | Upper USD price bound. Price-conditional only. | [optional]
**trigger_mint** | Option<**String**> | Mint whose USD price is checked against the band. Required for price_conditional. Must be a mint with a supported USD price (in practice the volatile leg / outputMint); a stablecoin such as USDC returns \"Trigger mint is not supported\". Ignored for time_based. | [optional]
**begin_fill_at** | Option<**String**> | ISO-8601 time to start the first round. Defaults to now, max 30 days in the future. | [optional]
**jl_enabled** | Option<**bool**> | Earn While You Wait. When true, idle funds earn Jupiter Lend yield between rounds. Requires orderType time_based, a supported stablecoin input, and a deposit crafted with jlMint. Default false. | [optional][default to false]
**jl_mint** | Option<**String**> | Jupiter Lend earn token for the input stablecoin (from GET /lend/v1/earn/tokens, the address whose assetAddress is the inputMint). Required when jlEnabled is true. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


