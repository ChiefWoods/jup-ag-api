# SendV1CraftClawbackPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**invite_pda** | **String** | - Program Derived Address derived by `\"invite\"` and the public key of the invite - Do note that, the invitePDA requires to be generated client side, [refer to docs](/docs/send) - Please handle invite code and secret key with highest security as user funds are involved  | 
**sender** | **String** | - Public key address of the sender - Since this address funded the invite, it will be the receiving address of the full amount  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


