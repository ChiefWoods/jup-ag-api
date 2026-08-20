# LendBorrowOperatePayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**vault_id** | **i32** | Target vault id. | 
**position_id** | **i32** | Existing position id. Use `0` when opening a new position. | 
**position_owner** | Option<**String**> | Optional position owner when operating on behalf of another wallet. | [optional]
**signer** | **String** | Transaction fee payer and signer wallet address. | 
**col_amount** | **String** | Collateral delta in supply-token base units. Positive deposits collateral, negative withdraws. Use -170141183460469231731687303715884105728 (MIN_I128) to withdraw all collateral. | 
**debt_amount** | **String** | Debt delta in borrow-token base units. Positive borrows, negative repays. Use -170141183460469231731687303715884105728 (MIN_I128) to repay all debt. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


