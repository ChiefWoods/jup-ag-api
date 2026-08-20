# LendTokenInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | - Lending ID, starts from `1`  | 
**address** | **String** | - Lending token address like `9BEcn9aPEmhSPbPQeFGjidRiEKki46fVQDyPpSQXPA2D`  | 
**name** | **String** | - Lending token name like Jupiter Lend USDC  | 
**symbol** | **String** | - Lending token symbol like jlUSDC  | 
**decimals** | **i32** |  | 
**asset_address** | **String** | - Lending token underlying asset address like `EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v`  | 
**asset** | [**models::LendAssetInfo**](LendAssetInfo.md) |  | 
**total_assets** | **String** | - Total available underlying mint from jlTokens (with accured interest)  | 
**total_supply** | **String** | - Total supply of jlTokens for this mint  | 
**convert_to_shares** | **String** | - Value of one 1 underlying token to jlToken, like 1 USDC = X jlUSDC - Used for conversion  | 
**convert_to_assets** | **String** | - Value of one 1 jlToken token to underlying mint, like 1 jlUSDC = X USDC - Used for conversion  | 
**rewards_rate** | **String** | - Rewards rate , in 1e4 decimals  | 
**supply_rate** | **String** | - Supply rate, in 1e4 decimals  | 
**total_rate** | **String** | - Total sum of rewards + supply rate  | 
**rebalance_difference** | **String** |  | 
**liquidity_supply_data** | [**models::LendLiquiditySupplyData**](LendLiquiditySupplyData.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


