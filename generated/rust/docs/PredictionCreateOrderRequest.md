# PredictionCreateOrderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**owner_pubkey** | Option<**String**> | Order owner public key | [optional]
**market_id** | Option<**String**> | Hashed or canonical market ID for buys | [optional]
**position_pubkey** | Option<**String**> | Required for sells | [optional]
**is_yes** | Option<**bool**> |  | [optional]
**is_buy** | **bool** |  | 
**contracts** | Option<[**models::PredictionCreateOrderRequestContracts**](PredictionCreateOrderRequestContracts.md)> |  | [optional]
**contracts_micro** | Option<[**models::PredictionCreateOrderRequestContractsMicro**](PredictionCreateOrderRequestContractsMicro.md)> |  | [optional]
**contracts_decimal** | Option<[**models::PredictionCreateOrderRequestContractsDecimal**](PredictionCreateOrderRequestContractsDecimal.md)> |  | [optional]
**deposit_amount** | Option<[**models::PredictionCreateOrderRequestDepositAmount**](PredictionCreateOrderRequestDepositAmount.md)> |  | [optional]
**deposit_mint** | Option<**String**> | Mint address for the deposit token | [optional][default to EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


