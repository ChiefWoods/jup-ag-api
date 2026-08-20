# PredictionExecuteResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | **Status** | Transaction execution result. (enum: Success, Failed) | 
**signature** | Option<**String**> | Transaction signature (base58) when execution succeeds. | [optional]
**error** | Option<**String**> | Error message if the transaction failed, or `null` on success. | [optional]
**request_id** | Option<**String**> | Echoed from the request when provided. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


