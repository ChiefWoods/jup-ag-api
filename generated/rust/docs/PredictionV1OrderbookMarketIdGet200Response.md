# PredictionV1OrderbookMarketIdGet200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**yes** | [**Vec<Vec<f64>>**](Vec.md) | YES-side levels sorted by price ascending. Each entry is a [price_cents, size] tuple. price_cents is rounded, so sub-cent levels show 0; size may be fractional. | 
**no** | [**Vec<Vec<f64>>**](Vec.md) | NO-side levels sorted by price ascending. Each entry is a [price_cents, size] tuple. price_cents is rounded, so sub-cent levels show 0; size may be fractional. | 
**yes_dollars** | [**Vec<Vec<serde_json::Value>>**](Vec.md) | YES-side levels with price as decimal dollar string. Each entry is a [price_string, size] tuple, e.g. [\"0.0100\", 346014]. | 
**no_dollars** | [**Vec<Vec<serde_json::Value>>**](Vec.md) | NO-side levels with price as decimal dollar string. Each entry is a [price_string, size] tuple, e.g. [\"0.0100\", 219]. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


