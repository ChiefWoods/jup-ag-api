# PredictionMarket

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**market_id** | **String** | Market identifier | 
**event_id** | Option<**String**> | Parent event identifier | [optional]
**provider** | Option<**Provider**> | Market data provider (enum: polymarket, gx, bisonfi) | [optional]
**title** | Option<**String**> | Market title | [optional]
**status** | **Status** | Current market status (enum: open, closed, cancelled) | 
**result** | Option<**Result**> | Market result. null until resolved, then \"yes\" or \"no\" (enum: yes, no) | 
**open_time** | **f64** | Unix timestamp (seconds) when the market opens | 
**close_time** | **f64** | Unix timestamp (seconds) when the market closes | 
**resolve_at** | Option<[**models::PredictionMarketResolveAt**](PredictionMarketResolveAt.md)> |  | 
**market_result_pubkey** | Option<**String**> | Public key of the market result account | [optional]
**image_url** | Option<**String**> | Market image URL | [optional]
**rules_primary** | Option<**String**> | Primary resolution rules | [optional]
**rules_secondary** | Option<**String**> | Secondary resolution rules | [optional]
**outcomes** | Option<**Vec<String>**> | Outcome labels for the market | [optional]
**market_options** | Option<[**Vec<models::PredictionV1TradesGet200ResponseDataInnerMarketOptionsInner>**](PredictionV1TradesGet200ResponseDataInnerMarketOptionsInner.md)> | Selectable options for team or multi-outcome markets | [optional]
**clob_token_ids** | Option<**Vec<String>**> | Underlying CLOB token IDs (provider-specific) | [optional]
**is_team_market** | Option<**bool**> | True for team or sports markets | [optional]
**team** | Option<**serde_json::Value**> | Team metadata, present for team markets | [optional]
**sports_line** | Option<**f64**> | Sports line or handicap, when applicable | [optional]
**sports_market_type** | Option<**String**> | Sports market type, when applicable | [optional]
**outcome_mint** | Option<**String**> | Outcome token mint for this market side (Jupiter Forecast / bisonfi). Buying the market gives you this token; it can also be traded directly through the Swap API. | [optional]
**outcome_token_program** | Option<**String**> | Token program that owns `outcomeMint`. Jupiter Forecast uses Token-2022 (`TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb`). | [optional]
**market_pda** | Option<**String**> | On-chain market account (Jupiter Forecast / bisonfi). | [optional]
**outcome_side** | Option<**String**> | Outcome side for Jupiter Forecast markets (`up` or `down`). | [optional]
**tradable** | Option<**bool**> | Whether this Jupiter Forecast side can be traded right now. The Discover example filters on this field. | [optional]
**lifecycle_status** | Option<**LifecycleStatus**> | Jupiter Forecast trading lifecycle: `open`, `resolving` after the close time, or `settled` after issuer resolution. Only `open` is tradable. (enum: open, resolving, settled) | [optional]
**pricing** | Option<[**models::PredictionMarketPricing**](PredictionMarketPricing.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


