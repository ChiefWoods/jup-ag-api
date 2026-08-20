# TriggerV2OrderHistoryItemEventsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**Type**> |  (enum: deposit, fill, withdrawal, cancelled, expired) | [optional]
**timestamp** | Option<**f64**> |  | [optional]
**tx_signature** | Option<**String**> |  | [optional]
**state** | Option<**State**> |  (enum: success, failed, pending) | [optional]
**mint** | Option<**String**> | Token mint address (present on deposit/withdrawal events) | [optional]
**amount** | Option<**String**> | Token amount in smallest unit (present on deposit/withdrawal events) | [optional]
**output_mint** | Option<**String**> | Output token mint address (present on fill events) | [optional]
**output_amount** | Option<**String**> | Output token amount in smallest unit (present on fill events) | [optional]
**order_context** | Option<**String**> | Order context identifier (present on fill events) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


