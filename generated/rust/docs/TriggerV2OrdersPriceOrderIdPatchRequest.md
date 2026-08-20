# TriggerV2OrdersPriceOrderIdPatchRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_type** | **OrderType** |  (enum: single, oco, otoco) | 
**trigger_price_usd** | Option<**f64**> | New trigger price. Cannot be set on a trailing stop loss order. | [optional]
**trailing_bps** | Option<**f64**> | New trailing stop loss trail distance in basis points (50-9000), for an order that is already trailing. | [optional]
**slippage_bps** | Option<**f64**> |  | [optional]
**tp_price_usd** | Option<**f64**> |  | [optional]
**sl_price_usd** | Option<**f64**> |  | [optional]
**tp_slippage_bps** | Option<**f64**> |  | [optional]
**sl_slippage_bps** | Option<**f64**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


