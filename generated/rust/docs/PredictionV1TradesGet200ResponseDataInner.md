# PredictionV1TradesGet200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Trade identifier (e.g. \"order-1906029\") | 
**owner_pubkey** | **String** |  | 
**market_id** | **String** |  | 
**message** | **String** |  | 
**timestamp** | **f64** |  | 
**action** | **Action** |  (enum: buy, sell) | 
**side** | **Side** |  (enum: yes, no) | 
**event_title** | **String** |  | 
**market_title** | **String** |  | 
**amount_usd** | **String** |  | 
**price_usd** | **String** |  | 
**event_image_url** | **String** |  | 
**event_id** | **String** |  | 
**is_team_market** | Option<**bool**> |  | [optional]
**market_options** | Option<[**Vec<models::PredictionV1TradesGet200ResponseDataInnerMarketOptionsInner>**](PredictionV1TradesGet200ResponseDataInnerMarketOptionsInner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


