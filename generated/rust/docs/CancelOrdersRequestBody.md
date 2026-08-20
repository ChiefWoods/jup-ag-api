# CancelOrdersRequestBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**maker** | **String** |  | [default to jdocuPgEAjMfihABsPgKEvYtsmMzjUHeq9LX4Hvs7f3]
**compute_unit_price** | Option<**String**> | In microlamports, defaults to 95th percentile of priority fees  | [optional][default to auto]
**orders** | Option<**Vec<String>**> | Array of orders to cancel, if none is provided, the API will generate cancel instructions for all open orders belonging to the maker  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


