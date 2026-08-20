# PredictionClaimPositionResponsePosition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**position_pubkey** | **String** | Position account public key | 
**market_pubkey** | **String** | Market account public key | 
**user_pubkey** | **String** | User public key requesting claim | 
**owner_pubkey** | **String** | Position owner public key | 
**is_yes** | **bool** |  | 
**contracts** | **String** | Legacy whole-contract quantity settled. Use contractsMicro or contractsDecimal for the exact fractional quantity. | 
**contracts_micro** | Option<**String**> | Contracts settled in micro-contract units (u64 as string, 1000000 = 1 contract) | [optional]
**contracts_decimal** | Option<**String**> | Contracts settled as a decimal string | [optional]
**payout_amount_usd** | **String** | Payout amount in micro USD (u64 as string) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


