# SendV1CraftSendPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**invite_signer** | **String** | - Public key address generated from the hash of the invite code - Do note that, the invite code requires to be generated client side, [refer to docs](/docs/send) - Please handle invite code and secret key with highest security as user funds are involved  | 
**sender** | **String** | - Public key address of the sender - This address will be funding the amount to be sent and the token accounts involved  | 
**amount** | **String** | - Amount in atomic value (before decimals) - E.g. to send 1 USDC is equivalent of `amount='1000000'` - Do note that there will be additional SOL amount being used in the transaction for transaction fees and token accounts  | 
**mint** | Option<**String**> | - Defaults to WSOL mint - Only pass in this parameter with other mint when required  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


