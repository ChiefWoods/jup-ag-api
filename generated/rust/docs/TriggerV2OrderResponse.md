# TriggerV2OrderResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Order UUID | 
**tx_signature** | **String** | On-chain signature of the deposit transaction | 
**deposit_confirmed** | **bool** | Whether the deposit has landed and the order is active | 
**message** | Option<**String**> | Additional context about the deposit state, present on idempotent retries | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


