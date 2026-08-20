# \PredictionApi

All URIs are relative to *https://api.jup.ag*

Method | HTTP request | Description
------------- | ------------- | -------------
[**prediction_v1_events_event_id_get**](PredictionApi.md#prediction_v1_events_event_id_get) | **GET** /prediction/v1/events/{eventId} | 
[**prediction_v1_events_event_id_markets_get**](PredictionApi.md#prediction_v1_events_event_id_markets_get) | **GET** /prediction/v1/events/{eventId}/markets | 
[**prediction_v1_events_event_id_markets_market_id_get**](PredictionApi.md#prediction_v1_events_event_id_markets_market_id_get) | **GET** /prediction/v1/events/{eventId}/markets/{marketId} | 
[**prediction_v1_events_event_id_score_get**](PredictionApi.md#prediction_v1_events_event_id_score_get) | **GET** /prediction/v1/events/{eventId}/score | 
[**prediction_v1_events_get**](PredictionApi.md#prediction_v1_events_get) | **GET** /prediction/v1/events | 
[**prediction_v1_events_scores_get**](PredictionApi.md#prediction_v1_events_scores_get) | **GET** /prediction/v1/events/scores | 
[**prediction_v1_events_search_get**](PredictionApi.md#prediction_v1_events_search_get) | **GET** /prediction/v1/events/search | 
[**prediction_v1_events_suggested_pubkey_get**](PredictionApi.md#prediction_v1_events_suggested_pubkey_get) | **GET** /prediction/v1/events/suggested/{pubkey} | 
[**prediction_v1_execute_post**](PredictionApi.md#prediction_v1_execute_post) | **POST** /prediction/v1/execute | Execute a signed order transaction
[**prediction_v1_forecast_get**](PredictionApi.md#prediction_v1_forecast_get) | **GET** /prediction/v1/forecast | 
[**prediction_v1_history_get**](PredictionApi.md#prediction_v1_history_get) | **GET** /prediction/v1/history | 
[**prediction_v1_leaderboards_get**](PredictionApi.md#prediction_v1_leaderboards_get) | **GET** /prediction/v1/leaderboards | 
[**prediction_v1_markets_market_id_get**](PredictionApi.md#prediction_v1_markets_market_id_get) | **GET** /prediction/v1/markets/{marketId} | 
[**prediction_v1_orderbook_market_id_get**](PredictionApi.md#prediction_v1_orderbook_market_id_get) | **GET** /prediction/v1/orderbook/{marketId} | 
[**prediction_v1_orders_get**](PredictionApi.md#prediction_v1_orders_get) | **GET** /prediction/v1/orders | 
[**prediction_v1_orders_order_pubkey_get**](PredictionApi.md#prediction_v1_orders_order_pubkey_get) | **GET** /prediction/v1/orders/{orderPubkey} | 
[**prediction_v1_orders_post**](PredictionApi.md#prediction_v1_orders_post) | **POST** /prediction/v1/orders | 
[**prediction_v1_orders_status_order_pubkey_get**](PredictionApi.md#prediction_v1_orders_status_order_pubkey_get) | **GET** /prediction/v1/orders/status/{orderPubkey} | 
[**prediction_v1_positions_delete**](PredictionApi.md#prediction_v1_positions_delete) | **DELETE** /prediction/v1/positions | 
[**prediction_v1_positions_get**](PredictionApi.md#prediction_v1_positions_get) | **GET** /prediction/v1/positions | 
[**prediction_v1_positions_position_pubkey_claim_post**](PredictionApi.md#prediction_v1_positions_position_pubkey_claim_post) | **POST** /prediction/v1/positions/{positionPubkey}/claim | 
[**prediction_v1_positions_position_pubkey_delete**](PredictionApi.md#prediction_v1_positions_position_pubkey_delete) | **DELETE** /prediction/v1/positions/{positionPubkey} | 
[**prediction_v1_positions_position_pubkey_get**](PredictionApi.md#prediction_v1_positions_position_pubkey_get) | **GET** /prediction/v1/positions/{positionPubkey} | 
[**prediction_v1_profiles_owner_pubkey_get**](PredictionApi.md#prediction_v1_profiles_owner_pubkey_get) | **GET** /prediction/v1/profiles/{ownerPubkey} | 
[**prediction_v1_profiles_owner_pubkey_pnl_history_get**](PredictionApi.md#prediction_v1_profiles_owner_pubkey_pnl_history_get) | **GET** /prediction/v1/profiles/{ownerPubkey}/pnl-history | 
[**prediction_v1_trades_get**](PredictionApi.md#prediction_v1_trades_get) | **GET** /prediction/v1/trades | 
[**prediction_v1_trading_status_get**](PredictionApi.md#prediction_v1_trading_status_get) | **GET** /prediction/v1/trading-status | 
[**prediction_v1_vault_info_get**](PredictionApi.md#prediction_v1_vault_info_get) | **GET** /prediction/v1/vault-info | 



## prediction_v1_events_event_id_get

> models::PredictionEvent prediction_v1_events_event_id_get(event_id, include_markets, include_all_markets)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** | Event identifier | [required] |
**include_markets** | Option<**bool**> |  |  |
**include_all_markets** | Option<**bool**> | Include all allowed sports market types (moneyline, spread, totals) and extra (e.g. Saba) markets mapped onto Polymarket events. Defaults to false (moneyline only). F1 events always include every sportsMarketType. |  |

### Return type

[**models::PredictionEvent**](Prediction_Event.md)

### Authorization

[Prediction_ApiKeyAuth](../README.md#Prediction_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prediction_v1_events_event_id_markets_get

> models::PredictionV1EventsEventIdMarketsGet200Response prediction_v1_events_event_id_markets_get(event_id, start, end)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** | Event identifier | [required] |
**start** | Option<**i32**> |  |  |
**end** | Option<**i32**> |  |  |

### Return type

[**models::PredictionV1EventsEventIdMarketsGet200Response**](_prediction_v1_events__eventId__markets_get_200_response.md)

### Authorization

[Prediction_ApiKeyAuth](../README.md#Prediction_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prediction_v1_events_event_id_markets_market_id_get

> models::PredictionMarket prediction_v1_events_event_id_markets_market_id_get(event_id, market_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** | Event identifier | [required] |
**market_id** | **String** | Market identifier | [required] |

### Return type

[**models::PredictionMarket**](Prediction_Market.md)

### Authorization

[Prediction_ApiKeyAuth](../README.md#Prediction_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prediction_v1_events_event_id_score_get

> models::PredictionGameScore prediction_v1_events_event_id_score_get(event_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** | Event identifier | [required] |

### Return type

[**models::PredictionGameScore**](Prediction_GameScore.md)

### Authorization

[Prediction_ApiKeyAuth](../README.md#Prediction_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prediction_v1_events_get

> models::PredictionV1EventsGet200Response prediction_v1_events_get(provider, include_markets, include_all_markets, start, end, category, subcategory, sort_by, sort_direction, filter, tags)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider** | Option<**String**> | Data provider for events (defaults to polymarket). Use `bisonfi` for Jupiter Forecast markets. |  |[default to polymarket]
**include_markets** | Option<**bool**> |  |  |
**include_all_markets** | Option<**bool**> | Include all allowed sports market types (moneyline, spread, totals) and extra (e.g. Saba) markets mapped onto Polymarket events. Defaults to false (moneyline only). F1 events always include every sportsMarketType. |  |
**start** | Option<**i32**> |  |  |
**end** | Option<**i32**> |  |  |
**category** | Option<**String**> |  |  |
**subcategory** | Option<[**PredictionV1EventsGetSubcategoryParameter**](PredictionV1EventsGetSubcategoryParameter.md)> |  |  |
**sort_by** | Option<**String**> | Sort field for events (volume or begin time). Pair with sortDirection to control ascending/descending. |  |
**sort_direction** | Option<**String**> | Direction for the chosen sort field. Defaults to desc for volume and asc for beginAt. |  |
**filter** | Option<**String**> | Apply named filters. Use `new` for events created in the last 24 hours, `live` for events that have begun, `trending` for events with recent trade activity, and `upcoming` for events that have not begun yet. |  |
**tags** | Option<**String**> | Filter events by tag (e.g. `soccer`). |  |

### Return type

[**models::PredictionV1EventsGet200Response**](_prediction_v1_events_get_200_response.md)

### Authorization

[Prediction_ApiKeyAuth](../README.md#Prediction_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prediction_v1_events_scores_get

> models::PredictionV1EventsScoresGet200Response prediction_v1_events_scores_get(event_ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_ids** | **String** | Comma-separated event IDs to fetch live scores for (max 100) | [required] |

### Return type

[**models::PredictionV1EventsScoresGet200Response**](_prediction_v1_events_scores_get_200_response.md)

### Authorization

[Prediction_ApiKeyAuth](../README.md#Prediction_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prediction_v1_events_search_get

> models::PredictionV1EventsSearchGet200Response prediction_v1_events_search_get(query, provider, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Search term matched against event titles | [required] |
**provider** | Option<**String**> | Data provider for events (defaults to polymarket). Use `bisonfi` for Jupiter Forecast markets. |  |[default to polymarket]
**limit** | Option<**i32**> |  |  |

### Return type

[**models::PredictionV1EventsSearchGet200Response**](_prediction_v1_events_search_get_200_response.md)

### Authorization

[Prediction_ApiKeyAuth](../README.md#Prediction_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prediction_v1_events_suggested_pubkey_get

> models::PredictionV1EventsSuggestedPubkeyGet200Response prediction_v1_events_suggested_pubkey_get(pubkey, provider)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pubkey** | **String** | Order public key to base suggested events on | [required] |
**provider** | Option<**String**> | Data provider for events (defaults to polymarket). Use `bisonfi` for Jupiter Forecast markets. |  |[default to polymarket]

### Return type

[**models::PredictionV1EventsSuggestedPubkeyGet200Response**](_prediction_v1_events_suggested__pubkey__get_200_response.md)

### Authorization

[Prediction_ApiKeyAuth](../README.md#Prediction_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prediction_v1_execute_post

> models::PredictionExecuteResponse prediction_v1_execute_post(prediction_execute_request)
Execute a signed order transaction

Submit a signed order transaction for execution. Pass the base64 `signedTransaction` together with the `execution.context` object returned by the order build (`POST /orders`, or `DELETE /positions/{positionPubkey}` for a sell), unchanged. Jupiter Forecast (bisonfi) orders execute as an atomic swap through this endpoint and the transaction signature is returned on success.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prediction_execute_request** | [**PredictionExecuteRequest**](PredictionExecuteRequest.md) |  | [required] |

### Return type

[**models::PredictionExecuteResponse**](Prediction_ExecuteResponse.md)

### Authorization

[Prediction_ApiKeyAuth](../README.md#Prediction_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prediction_v1_forecast_get

> models::PredictionV1ForecastGet200Response prediction_v1_forecast_get(market_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**market_id** | **String** | Market identifier to fetch the forecast for. Required; omitting it returns 400. | [required] |

### Return type

[**models::PredictionV1ForecastGet200Response**](_prediction_v1_forecast_get_200_response.md)

### Authorization

[Prediction_ApiKeyAuth](../README.md#Prediction_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prediction_v1_history_get

> models::PredictionV1HistoryGet200Response prediction_v1_history_get(start, end, owner_pubkey, id, position_pubkey)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**i32**> |  |  |
**end** | Option<**i32**> |  |  |
**owner_pubkey** | Option<**String**> | History owner public key |  |
**id** | Option<**i32**> | History event ID |  |
**position_pubkey** | Option<**String**> | Filter by position public key |  |

### Return type

[**models::PredictionV1HistoryGet200Response**](_prediction_v1_history_get_200_response.md)

### Authorization

[Prediction_ApiKeyAuth](../README.md#Prediction_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prediction_v1_leaderboards_get

> models::PredictionV1LeaderboardsGet200Response prediction_v1_leaderboards_get(period, limit, metric)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**period** | Option<**String**> |  |  |[default to all_time]
**limit** | Option<**i32**> |  |  |[default to 100]
**metric** | Option<**String**> |  |  |[default to pnl]

### Return type

[**models::PredictionV1LeaderboardsGet200Response**](_prediction_v1_leaderboards_get_200_response.md)

### Authorization

[Prediction_ApiKeyAuth](../README.md#Prediction_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prediction_v1_markets_market_id_get

> models::PredictionMarket prediction_v1_markets_market_id_get(market_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**market_id** | **String** | Market identifier | [required] |

### Return type

[**models::PredictionMarket**](Prediction_Market.md)

### Authorization

[Prediction_ApiKeyAuth](../README.md#Prediction_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prediction_v1_orderbook_market_id_get

> models::PredictionV1OrderbookMarketIdGet200Response prediction_v1_orderbook_market_id_get(market_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**market_id** | **String** | Kalshi or Polymarket market identifier | [required] |

### Return type

[**models::PredictionV1OrderbookMarketIdGet200Response**](_prediction_v1_orderbook__marketId__get_200_response.md)

### Authorization

[Prediction_ApiKeyAuth](../README.md#Prediction_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prediction_v1_orders_get

> models::PredictionV1OrdersGet200Response prediction_v1_orders_get(start, end, owner_pubkey)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**i32**> |  |  |
**end** | Option<**i32**> |  |  |
**owner_pubkey** | Option<**String**> |  |  |

### Return type

[**models::PredictionV1OrdersGet200Response**](_prediction_v1_orders_get_200_response.md)

### Authorization

[Prediction_ApiKeyAuth](../README.md#Prediction_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prediction_v1_orders_order_pubkey_get

> models::PredictionOrder prediction_v1_orders_order_pubkey_get(order_pubkey)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_pubkey** | **String** | Order account public key | [required] |

### Return type

[**models::PredictionOrder**](Prediction_Order.md)

### Authorization

[Prediction_ApiKeyAuth](../README.md#Prediction_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prediction_v1_orders_post

> models::PredictionCreateOrderResponse prediction_v1_orders_post(prediction_create_order_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prediction_create_order_request** | [**PredictionCreateOrderRequest**](PredictionCreateOrderRequest.md) |  | [required] |

### Return type

[**models::PredictionCreateOrderResponse**](Prediction_CreateOrderResponse.md)

### Authorization

[Prediction_ApiKeyAuth](../README.md#Prediction_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prediction_v1_orders_status_order_pubkey_get

> models::PredictionV1OrdersStatusOrderPubkeyGet200Response prediction_v1_orders_status_order_pubkey_get(order_pubkey)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_pubkey** | **String** | Order account public key | [required] |

### Return type

[**models::PredictionV1OrdersStatusOrderPubkeyGet200Response**](_prediction_v1_orders_status__orderPubkey__get_200_response.md)

### Authorization

[Prediction_ApiKeyAuth](../README.md#Prediction_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prediction_v1_positions_delete

> models::PredictionV1PositionsDelete200Response prediction_v1_positions_delete(prediction_close_all_positions_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prediction_close_all_positions_request** | [**PredictionCloseAllPositionsRequest**](PredictionCloseAllPositionsRequest.md) |  | [required] |

### Return type

[**models::PredictionV1PositionsDelete200Response**](_prediction_v1_positions_delete_200_response.md)

### Authorization

[Prediction_ApiKeyAuth](../README.md#Prediction_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prediction_v1_positions_get

> models::PredictionV1PositionsGet200Response prediction_v1_positions_get(start, end, owner_pubkey, market_pubkey, market_id, is_yes)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**i32**> |  |  |
**end** | Option<**i32**> |  |  |
**owner_pubkey** | Option<**String**> | Position owner public key |  |
**market_pubkey** | Option<**String**> |  |  |
**market_id** | Option<**String**> |  |  |
**is_yes** | Option<**String**> | Filter by position side (true = YES, false = NO) |  |

### Return type

[**models::PredictionV1PositionsGet200Response**](_prediction_v1_positions_get_200_response.md)

### Authorization

[Prediction_ApiKeyAuth](../README.md#Prediction_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prediction_v1_positions_position_pubkey_claim_post

> models::PredictionClaimPositionResponse prediction_v1_positions_position_pubkey_claim_post(position_pubkey, prediction_claim_position_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**position_pubkey** | **String** | Position account public key | [required] |
**prediction_claim_position_request** | [**PredictionClaimPositionRequest**](PredictionClaimPositionRequest.md) |  | [required] |

### Return type

[**models::PredictionClaimPositionResponse**](Prediction_ClaimPositionResponse.md)

### Authorization

[Prediction_ApiKeyAuth](../README.md#Prediction_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prediction_v1_positions_position_pubkey_delete

> models::PredictionCreateOrderResponse prediction_v1_positions_position_pubkey_delete(position_pubkey, prediction_close_position_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**position_pubkey** | **String** | Position account public key | [required] |
**prediction_close_position_request** | [**PredictionClosePositionRequest**](PredictionClosePositionRequest.md) |  | [required] |

### Return type

[**models::PredictionCreateOrderResponse**](Prediction_CreateOrderResponse.md)

### Authorization

[Prediction_ApiKeyAuth](../README.md#Prediction_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prediction_v1_positions_position_pubkey_get

> models::PredictionPosition prediction_v1_positions_position_pubkey_get(position_pubkey)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**position_pubkey** | **String** | Position account public key | [required] |

### Return type

[**models::PredictionPosition**](Prediction_Position.md)

### Authorization

[Prediction_ApiKeyAuth](../README.md#Prediction_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prediction_v1_profiles_owner_pubkey_get

> models::PredictionV1ProfilesOwnerPubkeyGet200Response prediction_v1_profiles_owner_pubkey_get(owner_pubkey)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner_pubkey** | **String** |  | [required] |

### Return type

[**models::PredictionV1ProfilesOwnerPubkeyGet200Response**](_prediction_v1_profiles__ownerPubkey__get_200_response.md)

### Authorization

[Prediction_ApiKeyAuth](../README.md#Prediction_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prediction_v1_profiles_owner_pubkey_pnl_history_get

> models::PredictionV1ProfilesOwnerPubkeyPnlHistoryGet200Response prediction_v1_profiles_owner_pubkey_pnl_history_get(owner_pubkey, interval, count)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner_pubkey** | **String** |  | [required] |
**interval** | Option<**String**> |  |  |[default to 1w]
**count** | Option<**i32**> |  |  |[default to 10]

### Return type

[**models::PredictionV1ProfilesOwnerPubkeyPnlHistoryGet200Response**](_prediction_v1_profiles__ownerPubkey__pnl_history_get_200_response.md)

### Authorization

[Prediction_ApiKeyAuth](../README.md#Prediction_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prediction_v1_trades_get

> models::PredictionV1TradesGet200Response prediction_v1_trades_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PredictionV1TradesGet200Response**](_prediction_v1_trades_get_200_response.md)

### Authorization

[Prediction_ApiKeyAuth](../README.md#Prediction_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prediction_v1_trading_status_get

> models::PredictionTradingStatusResponse prediction_v1_trading_status_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PredictionTradingStatusResponse**](Prediction_TradingStatusResponse.md)

### Authorization

[Prediction_ApiKeyAuth](../README.md#Prediction_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prediction_v1_vault_info_get

> models::PredictionV1VaultInfoGet200Response prediction_v1_vault_info_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PredictionV1VaultInfoGet200Response**](_prediction_v1_vault_info_get_200_response.md)

### Authorization

[Prediction_ApiKeyAuth](../README.md#Prediction_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

