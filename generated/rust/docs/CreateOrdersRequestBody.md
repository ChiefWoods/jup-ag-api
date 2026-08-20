# CreateOrdersRequestBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**input_mint** | **String** |  | [default to EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v]
**output_mint** | **String** |  | [default to So11111111111111111111111111111111111111112]
**maker** | **String** |  | [default to jdocuPgEAjMfihABsPgKEvYtsmMzjUHeq9LX4Hvs7f3]
**payer** | **String** |  | [default to jdocuPgEAjMfihABsPgKEvYtsmMzjUHeq9LX4Hvs7f3]
**params** | [**models::CreateOrdersRequestBodyParams**](CreateOrdersRequestBodyParams.md) |  | 
**compute_unit_price** | Option<**String**> | In microlamports, defaults to 95th percentile of priority fees  | [optional][default to auto]
**fee_account** | Option<**String**> | - A token account (via the Referral Program) that will receive the fees - Refer to [Referral Program Github](https://github.com/TeamRaccoons/referral/tree/main) for more information  | [optional]
**wrap_and_unwrap_sol** | Option<**bool**> | If either input or output mint is native SOL | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


