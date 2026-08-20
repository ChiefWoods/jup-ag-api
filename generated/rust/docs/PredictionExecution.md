# PredictionExecution

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**endpoint** | **String** | Relative execute path. Resolves to `https://api.jup.ag/prediction/v1/execute`. | 
**context** | **std::collections::HashMap<String, serde_json::Value>** | Opaque execution context. Pass this object unchanged to `POST /execute`. Includes a `type` field (`create_order` for keeper-filled orders, `bisonfi_swap` for Jupiter Forecast orders). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


