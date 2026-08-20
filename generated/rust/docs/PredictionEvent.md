# PredictionEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_id** | **String** | Application-level identifier | 
**is_active** | **bool** | Whether the event is active (listed on events) | 
**is_live** | **bool** | Whether the event is currently live | 
**category** | **String** | The category for the event, allowed values: all, crypto, sports, politics, esports, culture, economics, tech | 
**subcategory** | **String** | The subcategory for the event | 
**tags** | Option<**Vec<String>**> | Tags associated with the event | [optional]
**metadata** | Option<[**models::PredictionEventMetadata**](PredictionEventMetadata.md)> |  | [optional]
**markets** | Option<[**Vec<models::PredictionMarket>**](PredictionMarket.md)> |  | [optional]
**volume_usd** | **String** | Total volume for the event in micro USD | 
**volume24hr** | Option<**String**> | Rolling 24-hour volume in micro USD | [optional]
**live_score** | Option<**serde_json::Value**> | Live score data for sports events; null when no score is available | [optional]
**close_condition** | **String** | Close condition for the event | 
**begin_at** | Option<**String**> | Unix timestamp (seconds) when the event begins | 
**rules_pdf** | **String** | Document to full rules for the event | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


