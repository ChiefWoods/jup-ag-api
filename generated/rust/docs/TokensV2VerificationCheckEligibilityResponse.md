# TokensV2VerificationCheckEligibilityResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**token_exists** | **bool** | Whether the token exists on-chain | 
**is_verified** | **bool** | Whether the token is already verified | 
**can_verify** | **bool** | Whether a verification request can be submitted | 
**can_metadata** | **bool** | Whether a metadata update can be submitted | 
**verification_error** | Option<**String**> | Reason verification is not allowed (only present when there is a specific error) | [optional]
**metadata_error** | Option<**String**> | Reason metadata update is not allowed (only present when there is a specific error) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


