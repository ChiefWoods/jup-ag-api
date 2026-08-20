# LendBorrowBorrowPosition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | NFT / position id. | 
**vault_id** | **i32** |  | 
**address** | **String** | Position NFT mint address. | 
**supply** | **String** |  | 
**before_supply** | **String** |  | 
**borrow** | **String** |  | 
**before_borrow** | **String** |  | 
**is_liquidated** | **bool** |  | 
**supply_liquidation** | **String** |  | 
**borrow_liquidation** | **String** |  | 
**is_supply_position** | **bool** |  | 
**tick** | **i32** |  | 
**tick_id** | **i32** |  | 
**dust_borrow** | **String** |  | 
**owner_address** | **String** |  | 
**vault** | [**models::LendBorrowBorrowVault**](LendBorrowBorrowVault.md) |  | 
**config** | **std::collections::HashMap<String, String>** | Key/value position config from the database, keyed in camelCase. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


