# CreateOrdersRequestBodyParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**making_amount** | **String** | Amount of input mint to swap | [default to 100000000]
**taking_amount** | **String** | Amount of output mint to receive | [default to 50000000]
**expired_at** | Option<**String**> |  | [optional]
**slippage_bps** | Option<**String**> | Amount of slippage the order can be executed with  | [optional][default to 0]
**fee_bps** | Option<**String**> | Requires the `feeAccount` parameter, the amount of fees in bps that will be sent to the fee account  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


