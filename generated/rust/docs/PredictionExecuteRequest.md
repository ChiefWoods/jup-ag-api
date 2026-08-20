# PredictionExecuteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**signed_transaction** | **String** | Base64-encoded signed transaction from the order build. | 
**context** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Optional. The `execution.context` object from the order build, passed unchanged. Validated against the signed transaction when present. For a Forecast swap it carries `jupiterSwapRequestId` and `ownerPubkey`. | [optional]
**request_id** | Option<**String**> | Optional client correlation ID, echoed back in the response. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


