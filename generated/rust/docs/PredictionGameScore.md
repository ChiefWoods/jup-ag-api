# PredictionGameScore

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_id** | **String** | Application-level event identifier | 
**game_id** | **String** |  | 
**league_abbreviation** | Option<**String**> |  | [optional]
**home_team** | Option<**String**> |  | [optional]
**away_team** | Option<**String**> |  | [optional]
**score** | Option<**String**> | Raw score string from the provider (e.g. \"2-0\") | [optional]
**period** | Option<**String**> | Current period (e.g. \"2H\") | [optional]
**elapsed** | Option<**String**> | Elapsed time within the game (e.g. \"90\") | [optional]
**status** | Option<**String**> | Provider status string (e.g. \"InProgress\") | [optional]
**live** | **bool** | True while the game is in progress | 
**ended** | **bool** | True once the game has finished | 
**finished_timestamp** | Option<**String**> | ISO 8601 timestamp when the game finished, if ended | [optional]
**updated_at** | **String** | ISO 8601 timestamp of the last score update | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


