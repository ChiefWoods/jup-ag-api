# LendBorrowToken

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | **String** | SPL token mint address. | 
**chain_id** | **String** |  | 
**name** | **String** |  | 
**symbol** | **String** |  | 
**ui_symbol** | **String** |  | 
**decimals** | **i32** |  | 
**logo_url** | Option<**String**> |  | [optional]
**price** | **String** | Token price as a decimal string. | 
**coingecko_id** | Option<**String**> |  | [optional]
**updated_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**staking_apr** | Option<**f64**> | Present on vault supply/borrow tokens when applicable. | [optional]
**native_stake_vote_account** | Option<**String**> | Present on native-stake collateral tokens when applicable. | [optional]
**scaled_ui_config** | Option<[**models::LendBorrowTokenScaledUiConfig**](LendBorrowTokenScaledUiConfig.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


