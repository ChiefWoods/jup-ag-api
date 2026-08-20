# StudioCreateDbcTransactionRequestBodyBuildCurveByMarketCapParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**quote_mint** | **String** | - Quote mint address - Either USDC, SOL, or JUP  | 
**initial_market_cap** | **f64** | - Initial market cap in quote token units - 16000 passed in = 16000 USDC/SOL/JUP initial market cap  | 
**migration_market_cap** | **f64** | - Migration market cap in quote token units - 69000 passed in = 69000 USDC/SOL/JUP migration market cap  | 
**token_quote_decimal** | **TokenQuoteDecimal** | - Token quote decimal places  (enum: 6, 9) | 
**locked_vesting_param** | [**models::StudioCreateDbcTransactionRequestBodyBuildCurveByMarketCapParamLockedVestingParam**](StudioCreateDBCTransactionRequestBodyBuildCurveByMarketCapParamLockedVestingParam.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


