# StudioCreateDbcTransactionResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transaction** | **String** | Base64-encoded transaction ready for signing | 
**mint** | **String** | Generated token mint address | 
**image_presigned_url** | **String** | - Presigned URL for image upload via `PUT` request - This is for you to make a PUT request to upload your token image to the on-chain uri metadata - [Refer to the Token Metadata section for better understanding](/docs/studio/create-token#token-metadata)  | 
**metadata_presigned_url** | **String** | - Presigned URL for metadata upload via `PUT` request - This is for you to make a PUT request to upload your token metadata to the on-chain uri metadata - [Refer to the Token Metadata section for better understanding](/docs/studio/create-token#token-metadata)  | 
**image_url** | **String** | Final image URL | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


