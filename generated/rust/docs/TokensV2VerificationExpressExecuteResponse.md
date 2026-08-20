# TokensV2VerificationExpressExecuteResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | **Status** | Whether the transaction was executed successfully (enum: Success, Failed) | 
**signature** | Option<**String**> | Transaction signature on-chain (present on success) | [optional]
**error** | Option<**String**> | Error message (present on failure) | [optional]
**code** | Option<**f64**> | Error code (present on failure) | [optional]
**total_time** | **f64** | Time taken to execute in milliseconds | 
**verification_created** | **bool** | Whether a verification request was created | 
**metadata_created** | **bool** | Whether a metadata update request was created | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


