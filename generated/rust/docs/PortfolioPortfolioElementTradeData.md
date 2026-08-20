# PortfolioPortfolioElementTradeData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assets** | [**models::PortfolioPortfolioElementTradeDataAssets**](PortfolioPortfolioElementTradeDataAssets.md) |  | 
**input_address** | **String** |  | 
**output_address** | **String** |  | 
**initial_input_amount** | **f64** |  | 
**withdrawn_output_amount** | **f64** |  | 
**expected_output_amount** | Option<**f64**> |  | [optional]
**input_price** | **f64** |  | 
**output_price** | **f64** |  | 
**filled_percentage** | **f64** | Filled percentage between 0 and 1 | 
**created_at** | Option<**f64**> | Created at timestamp in ms | [optional]
**expire_at** | Option<**f64**> | Expire at timestamp in ms | [optional]
**recurring** | Option<[**models::PortfolioPortfolioElementTradeDataRecurring**](PortfolioPortfolioElementTradeDataRecurring.md)> |  | [optional]
**contract** | Option<**String**> |  | [optional]
**r#ref** | Option<**String**> | Account address | [optional]
**source_refs** | Option<[**Vec<models::PortfolioSourceRef>**](PortfolioSourceRef.md)> |  | [optional]
**link** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


