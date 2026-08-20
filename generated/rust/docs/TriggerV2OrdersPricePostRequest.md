# TriggerV2OrdersPricePostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_type** | **OrderType** |  (enum: single, oco, otoco) | 
**deposit_request_id** | **String** | From /deposit/craft response | 
**deposit_signed_tx** | **String** | Base64-encoded signed deposit transaction | 
**user_pubkey** | **String** |  | 
**input_mint** | **String** |  | 
**input_amount** | **String** | Amount in smallest unit | 
**output_mint** | **String** |  | 
**trigger_mint** | **String** | Token mint to monitor for price | 
**trigger_condition** | Option<**TriggerCondition**> | Single/OTOCO only (enum: above, below) | [optional]
**trigger_price_usd** | Option<**f64**> | Single/OTOCO trigger price. For single orders, required unless trailingBps is set. | [optional]
**trailing_bps** | Option<**f64**> | Trailing stop loss trail distance in basis points (50-9000). Single orders only; set instead of triggerPriceUsd. Direction from triggerCondition: below = sell-below (triggerMint === inputMint), above = buy-above (triggerMint === outputMint). | [optional]
**slippage_bps** | Option<**f64**> | Slippage tolerance (0-10000 bps) | [optional]
**tp_price_usd** | Option<**f64**> | Take-profit price (OCO/OTOCO) | [optional]
**sl_price_usd** | Option<**f64**> | Stop-loss price (OCO/OTOCO) | [optional]
**tp_slippage_bps** | Option<**f64**> | Take-profit slippage (OCO/OTOCO) | [optional]
**sl_slippage_bps** | Option<**f64**> | Stop-loss slippage (OCO/OTOCO) | [optional]
**expires_at** | **f64** | Expiration timestamp (milliseconds) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


