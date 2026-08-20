# TriggerV2DepositCraftPost200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transaction** | **String** | Base64-encoded unsigned transaction | 
**request_id** | **String** | Deposit request ID (used when creating the order) | 
**receiver_address** | **String** | Vault public key receiving the deposit | 
**mint** | **String** | Token mint being deposited | 
**amount** | **String** | Deposit amount in smallest unit | 
**token_decimals** | **f64** | Token decimals | 
**input_token_account** | Option<**String**> | Deterministic per-order input token account. Returned for deposits crafted with an orderSubType. | [optional]
**output_token_account** | Option<**String**> | Deterministic per-order output token account. Returned only when orderSubType is otoco. | [optional]
**jl_token_account** | Option<**String**> | Jupiter Lend position account. Returned instead of inputTokenAccount when the deposit is crafted with jlMint (Earn While You Wait). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


