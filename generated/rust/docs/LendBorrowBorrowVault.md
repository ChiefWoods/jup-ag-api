# LendBorrowBorrowVault

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | 
**address** | **String** |  | 
**supply_token** | [**models::LendBorrowToken**](LendBorrowToken.md) |  | 
**borrow_token** | [**models::LendBorrowToken**](LendBorrowToken.md) |  | 
**total_supply** | Option<**String**> |  | [optional]
**total_supply_liquidity** | Option<**String**> |  | [optional]
**total_borrow** | Option<**String**> |  | [optional]
**total_borrow_liquidity** | Option<**String**> |  | [optional]
**absorbed_supply** | Option<**String**> |  | [optional]
**absorbed_borrow** | Option<**String**> |  | [optional]
**supply_rate_magnifier** | Option<**String**> |  | [optional]
**borrow_rate_magnifier** | Option<**String**> |  | [optional]
**borrow_fee** | Option<**String**> |  | [optional]
**collateral_factor** | Option<**String**> |  | [optional]
**liquidation_threshold** | Option<**String**> |  | [optional]
**liquidation_max_limit** | Option<**String**> |  | [optional]
**liquidation_penalty** | Option<**String**> |  | [optional]
**withdrawal_gap** | Option<**String**> |  | [optional]
**supply_rate** | Option<**String**> |  | [optional]
**supply_rate_liquidity** | Option<**String**> |  | [optional]
**borrow_rate** | Option<**String**> |  | [optional]
**borrow_rate_liquidity** | Option<**String**> |  | [optional]
**withdraw_limit** | Option<**String**> |  | [optional]
**withdrawable_until_limit** | Option<**String**> |  | [optional]
**withdrawable** | Option<**String**> |  | [optional]
**borrow_limit** | Option<**String**> |  | [optional]
**borrowable_until_limit** | Option<**String**> |  | [optional]
**borrowable** | Option<**String**> |  | [optional]
**borrow_limit_utilization** | Option<**String**> |  | [optional]
**minimum_borrowing** | Option<**String**> |  | [optional]
**total_positions** | Option<**i32**> |  | [optional]
**oracle** | Option<**String**> |  | [optional]
**oracle_price** | Option<**String**> |  | [optional]
**oracle_price_operate** | Option<**String**> |  | [optional]
**oracle_price_liquidate** | Option<**String**> |  | [optional]
**oracle_sources** | Option<[**Vec<models::LendBorrowOracleSource>**](LendBorrowOracleSource.md)> |  | [optional]
**oracle_timestamp** | Option<**i64**> |  | [optional]
**liquidity_supply_data** | Option<[**models::LendBorrowLiquiditySupplyData**](LendBorrowLiquiditySupplyData.md)> |  | [optional]
**liquidity_borrow_data** | Option<[**models::LendBorrowLiquidityBorrowData**](LendBorrowLiquidityBorrowData.md)> |  | [optional]
**rewards** | Option<[**Vec<models::LendBorrowMarketReward>**](LendBorrowMarketReward.md)> |  | [optional]
**top_tick** | Option<**i32**> |  | [optional]
**metadata** | [**models::LendBorrowVaultMetadata**](LendBorrowVaultMetadata.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


