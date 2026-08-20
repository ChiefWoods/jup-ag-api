# SwapV1QuoteResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**input_mint** | **String** |  | 
**in_amount** | **String** |  | 
**output_mint** | **String** |  | 
**out_amount** | **String** | - Best output amount after deducting AMM fees and platform fees - Does not account for slippage  | 
**other_amount_threshold** | **String** | - Minimum acceptable output amount — `outAmount` after applying `slippageBps` tolerance - Not used by `/swap` endpoint to build transaction  | 
**swap_mode** | [**models::SwapV1SwapMode**](SwapV1SwapMode.md) |  | 
**slippage_bps** | **i32** |  | 
**platform_fee** | Option<[**models::SwapV1PlatformFee**](SwapV1PlatformFee.md)> |  | [optional]
**price_impact_pct** | **String** |  | 
**route_plan** | [**Vec<models::SwapV1RoutePlanStep>**](SwapV1RoutePlanStep.md) |  | 
**context_slot** | Option<**i32**> |  | [optional]
**time_taken** | Option<**f64**> |  | [optional]
**most_reliable_amms_quote_report** | Option<[**models::SwapV1QuoteResponseMostReliableAmmsQuoteReport**](SwapV1QuoteResponseMostReliableAmmsQuoteReport.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


