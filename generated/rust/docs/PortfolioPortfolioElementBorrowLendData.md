# PortfolioPortfolioElementBorrowLendData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**value** | **f64** | The value of the portfolio element in USD | 
**supplied_assets** | [**Vec<models::PortfolioPortfolioAsset>**](PortfolioPortfolioAsset.md) |  | 
**borrowed_assets** | [**Vec<models::PortfolioPortfolioAsset>**](PortfolioPortfolioAsset.md) |  | 
**reward_assets** | [**Vec<models::PortfolioPortfolioAsset>**](PortfolioPortfolioAsset.md) |  | 
**unsettled** | Option<[**models::PortfolioPortfolioElementBorrowLendDataUnsettled**](PortfolioPortfolioElementBorrowLendDataUnsettled.md)> |  | [optional]
**supplied_value** | **f64** |  | 
**borrowed_value** | **f64** |  | 
**reward_value** | **f64** |  | 
**supplied_yields** | [**Vec<Vec<models::PortfolioYield>>**](Vec.md) |  | 
**borrowed_yields** | [**Vec<Vec<models::PortfolioYield>>**](Vec.md) |  | 
**health_ratio** | Option<**f64**> | Health ratio (1 = full health, 0 or below = can be liquidated, null = unknown) | 
**expire_on** | Option<**f64**> | Date (in ms) when the loan will expire | [optional]
**contract** | Option<**String**> |  | [optional]
**r#ref** | Option<**String**> | Account address | [optional]
**source_refs** | Option<[**Vec<models::PortfolioSourceRef>**](PortfolioSourceRef.md)> |  | [optional]
**link** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


