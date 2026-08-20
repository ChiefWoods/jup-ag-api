# TriggerV2OrderHistoryItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**order_type** | **OrderType** |  (enum: single, oco, otoco) | 
**order_state** | **OrderState** |  (enum: pending, open, executing, filled, pending_withdraw, cancelled, expired, failed) | 
**raw_state** | Option<**String**> |  | [optional]
**user_pubkey** | **String** |  | 
**privy_wallet_pubkey** | Option<**String**> |  | [optional]
**input_mint** | **String** |  | 
**initial_input_amount** | Option<**String**> |  | [optional]
**remaining_input_amount** | Option<**String**> |  | [optional]
**output_mint** | **String** |  | 
**trigger_mint** | Option<**String**> |  | [optional]
**trigger_condition** | Option<**TriggerCondition**> |  (enum: above, below) | [optional]
**trigger_price_usd** | Option<**f64**> |  | [optional]
**slippage_bps** | Option<**f64**> |  | [optional]
**expires_at** | Option<**f64**> |  | [optional]
**created_at** | Option<**f64**> |  | [optional]
**updated_at** | Option<**f64**> |  | [optional]
**triggered_at** | Option<**f64**> |  | [optional]
**output_amount** | Option<**String**> |  | [optional]
**input_used** | Option<**String**> |  | [optional]
**fill_percent** | Option<**f64**> |  | [optional]
**trailing_bps** | Option<**f64**> | Trailing stop loss trail distance in basis points. Null for non-trailing orders. | [optional]
**high_watermark** | Option<**f64**> | Highest observed trigger-mint price (sell-below trailing). | [optional]
**low_watermark** | Option<**f64**> | Lowest observed trigger-mint price (buy-above trailing). | [optional]
**events** | Option<[**Vec<models::TriggerV2OrderHistoryItemEventsInner>**](TriggerV2OrderHistoryItemEventsInner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


