# RecurringOrderHistoryResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | **String** |  | 
**confirmed_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**fee_amount** | **String** |  | 
**fee_mint** | **String** |  | 
**input_amount** | **String** |  | 
**input_mint** | **String** |  | 
**keeper** | **String** |  | 
**order_key** | **String** |  | 
**output_amount** | **String** |  | 
**output_mint** | **String** |  | 
**product_meta** | Option<**serde_json::Value**> | Extra metadata for sub-products which will never be queried  with filters.   See structs `DbOrderHistoryValueAverageMeta` and `DbOrderHistoryPerpetualMeta`  to craft the JSONB object.  | [optional]
**raw_fee_amount** | **String** |  | 
**raw_input_amount** | **String** |  | 
**raw_output_amount** | **String** |  | 
**tx_id** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


