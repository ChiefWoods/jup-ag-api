# TriggerV2DepositCraftPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**input_mint** | **String** | Mint address of the token to deposit | 
**output_mint** | **String** | Mint address of the desired output token | 
**user_address** | **String** | Your wallet public key | 
**amount** | **String** | Amount in smallest unit (e.g. lamports for SOL) | 
**order_type** | **OrderType** | Order type for the deposit. Use price for price trigger orders, or dca for dollar-cost averaging orders. (enum: price, dca) | 
**order_sub_type** | Option<**OrderSubType**> | Required when orderType is price (single for standalone orders, oco for OCO orders, otoco for OTOCO orders). Must be omitted when orderType is dca. (enum: single, oco, otoco) | [optional]
**jl_mint** | Option<**String**> | For Earn While You Wait DCA orders only. The Jupiter Lend earn token for the input stablecoin (from GET /lend/v1/earn/tokens). Crafts a deposit into Jupiter Lend; the response returns jlTokenAccount instead of inputTokenAccount. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


