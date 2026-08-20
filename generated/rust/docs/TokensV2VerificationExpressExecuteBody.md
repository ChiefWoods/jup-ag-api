# TokensV2VerificationExpressExecuteBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transaction** | **String** | Base64-encoded signed transaction from the craft-txn step | 
**request_id** | **String** | requestId returned by craft-txn | 
**sender_address** | **String** | Solana wallet address of the sender | 
**token_id** | **String** | Token mint address to verify | 
**twitter_handle** | **String** | Twitter/X handle of the token project | 
**sender_twitter_handle** | Option<**String**> | Twitter/X handle of the submitter (optional) | [optional]
**description** | **String** | Reason for verification request | 
**token_metadata** | Option<[**models::TokensV2VerificationTokenMetadataInput**](TokensV2VerificationTokenMetadataInput.md)> |  | [optional]
**payment_currency** | Option<[**models::TokensV2VerificationPaymentCurrency**](TokensV2VerificationPaymentCurrency.md)> | Payment currency. Defaults to JUP (direct transfer). SOL/USDC/JUPUSD execute the Ultra swap crafted in the previous step. | [optional]
**payment_amount** | Option<**String**> | Atomic input amount paid. Required when paymentCurrency is not JUP. | [optional]
**jup_output_amount** | Option<**String**> | Atomic JUP output from the craft quote (CraftTxnResponse.amount), echoed back so revenue tracks the actual JUP collected on swap paths. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


