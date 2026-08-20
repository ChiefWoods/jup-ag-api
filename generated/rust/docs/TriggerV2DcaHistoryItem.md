# TriggerV2DcaHistoryItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | DCA order UUID | 
**request_id** | Option<**String**> |  | [optional]
**user_pubkey** | **String** |  | 
**vault_pubkey** | **String** |  | 
**input_mint** | **String** |  | 
**output_mint** | **String** |  | 
**input_amount_initial** | Option<**String**> | Total deposit amount in smallest units | [optional]
**input_amount_remaining** | Option<**String**> | Amount still held in the vault | [optional]
**amount_per_round** | Option<**String**> | Amount swapped each round (the last round may include dust) | [optional]
**output_amount_total** | Option<**String**> | Total output received across all successful rounds | [optional]
**input_amount_used** | Option<**String**> | Total input consumed by successful rounds | [optional]
**order_type** | **OrderType** |  (enum: time_based, price_conditional) | 
**min_price_usd** | Option<**f64**> |  | [optional]
**max_price_usd** | Option<**f64**> |  | [optional]
**trigger_mint** | Option<**String**> |  | [optional]
**retry_window_seconds** | Option<**f64**> | Retry window per round in seconds, clamp(intervalSeconds/2, 30, 7200) | [optional]
**jl_enabled** | Option<**bool**> | Whether Earn While You Wait is on for this order (idle funds earn Jupiter Lend yield between rounds). | [optional]
**jl_yield_usd** | Option<**f64**> | Yield earned so far in USD for an Earn While You Wait order, from the ~15 min price cache. null when not JL-enabled or a required price is uncached. | [optional]
**number_of_rounds** | Option<**f64**> |  | [optional]
**interval_seconds** | Option<**f64**> |  | [optional]
**begin_fill_at** | Option<**String**> | ISO-8601 time the first round is scheduled | [optional]
**next_fill_at** | Option<**String**> | ISO-8601 time of the next scheduled round | [optional]
**last_fill_at** | Option<**String**> |  | [optional]
**rounds_filled** | Option<**f64**> |  | [optional]
**fill_percent** | Option<**f64**> | Progress from 0 to 1 | [optional]
**state** | **State** |  (enum: depositing, deposit_failed, active, executing, withdrawing, completed, cancelled) | 
**display_state** | **DisplayState** |  (enum: pending, active, executing, pending_withdraw, completed, cancelled, failed) | 
**created_at** | Option<**String**> |  | [optional]
**updated_at** | Option<**String**> |  | [optional]
**events** | Option<[**Vec<models::TriggerV2DcaHistoryItemEventsInner>**](TriggerV2DcaHistoryItemEventsInner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


