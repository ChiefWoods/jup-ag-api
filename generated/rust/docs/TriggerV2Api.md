# \TriggerV2Api

All URIs are relative to *https://api.jup.ag*

Method | HTTP request | Description
------------- | ------------- | -------------
[**trigger_v2_auth_challenge_post**](TriggerV2Api.md#trigger_v2_auth_challenge_post) | **POST** /trigger/v2/auth/challenge | Request authentication challenge
[**trigger_v2_auth_verify_post**](TriggerV2Api.md#trigger_v2_auth_verify_post) | **POST** /trigger/v2/auth/verify | Verify signed challenge and receive JWT
[**trigger_v2_deposit_craft_post**](TriggerV2Api.md#trigger_v2_deposit_craft_post) | **POST** /trigger/v2/deposit/craft | Craft deposit transaction
[**trigger_v2_orders_dca_cancel_id_post**](TriggerV2Api.md#trigger_v2_orders_dca_cancel_id_post) | **POST** /trigger/v2/orders/dca/cancel/{id} | Initiate DCA cancellation
[**trigger_v2_orders_dca_confirm_cancel_id_post**](TriggerV2Api.md#trigger_v2_orders_dca_confirm_cancel_id_post) | **POST** /trigger/v2/orders/dca/confirm-cancel/{id} | Confirm DCA cancellation
[**trigger_v2_orders_dca_post**](TriggerV2Api.md#trigger_v2_orders_dca_post) | **POST** /trigger/v2/orders/dca | Create DCA order
[**trigger_v2_orders_history_dca_get**](TriggerV2Api.md#trigger_v2_orders_history_dca_get) | **GET** /trigger/v2/orders/history/dca | List DCA orders
[**trigger_v2_orders_history_dca_id_get**](TriggerV2Api.md#trigger_v2_orders_history_dca_id_get) | **GET** /trigger/v2/orders/history/dca/{id} | Get DCA order
[**trigger_v2_orders_history_get**](TriggerV2Api.md#trigger_v2_orders_history_get) | **GET** /trigger/v2/orders/history | Get order history
[**trigger_v2_orders_price_cancel_order_id_post**](TriggerV2Api.md#trigger_v2_orders_price_cancel_order_id_post) | **POST** /trigger/v2/orders/price/cancel/{orderId} | Initiate order cancellation
[**trigger_v2_orders_price_confirm_cancel_order_id_post**](TriggerV2Api.md#trigger_v2_orders_price_confirm_cancel_order_id_post) | **POST** /trigger/v2/orders/price/confirm-cancel/{orderId} | Confirm order cancellation
[**trigger_v2_orders_price_order_id_patch**](TriggerV2Api.md#trigger_v2_orders_price_order_id_patch) | **PATCH** /trigger/v2/orders/price/{orderId} | Update price order
[**trigger_v2_orders_price_post**](TriggerV2Api.md#trigger_v2_orders_price_post) | **POST** /trigger/v2/orders/price | Create price order
[**trigger_v2_vault_get**](TriggerV2Api.md#trigger_v2_vault_get) | **GET** /trigger/v2/vault | Get vault info
[**trigger_v2_vault_register_get**](TriggerV2Api.md#trigger_v2_vault_register_get) | **GET** /trigger/v2/vault/register | Register a new vault



## trigger_v2_auth_challenge_post

> models::TriggerV2AuthChallengePost200Response trigger_v2_auth_challenge_post(trigger_v2_auth_challenge_post_request)
Request authentication challenge

Request a challenge to sign with your wallet. Use `message` for standard wallets or `transaction` for hardware wallets that only support transaction signing. Challenges expire after 5 minutes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trigger_v2_auth_challenge_post_request** | [**TriggerV2AuthChallengePostRequest**](TriggerV2AuthChallengePostRequest.md) |  | [required] |

### Return type

[**models::TriggerV2AuthChallengePost200Response**](_trigger_v2_auth_challenge_post_200_response.md)

### Authorization

[TriggerV2_ApiKeyAuth](../README.md#TriggerV2_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_v2_auth_verify_post

> models::TriggerV2AuthVerifyPost200Response trigger_v2_auth_verify_post(trigger_v2_auth_verify_post_request)
Verify signed challenge and receive JWT

Submit the signed challenge to receive a JWT token valid for 24 hours. Include the JWT in all subsequent authenticated requests via the `Authorization: Bearer` header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trigger_v2_auth_verify_post_request** | [**TriggerV2AuthVerifyPostRequest**](TriggerV2AuthVerifyPostRequest.md) |  | [required] |

### Return type

[**models::TriggerV2AuthVerifyPost200Response**](_trigger_v2_auth_verify_post_200_response.md)

### Authorization

[TriggerV2_ApiKeyAuth](../README.md#TriggerV2_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_v2_deposit_craft_post

> models::TriggerV2DepositCraftPost200Response trigger_v2_deposit_craft_post(trigger_v2_deposit_craft_post_request)
Craft deposit transaction

Build an unsigned transaction to deposit tokens from your wallet into your vault. The vault address is automatically resolved from your JWT token. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trigger_v2_deposit_craft_post_request** | [**TriggerV2DepositCraftPostRequest**](TriggerV2DepositCraftPostRequest.md) |  | [required] |

### Return type

[**models::TriggerV2DepositCraftPost200Response**](_trigger_v2_deposit_craft_post_200_response.md)

### Authorization

[TriggerV2_ApiKeyAuth](../README.md#TriggerV2_ApiKeyAuth), [TriggerV2_BearerAuth](../README.md#TriggerV2_BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_v2_orders_dca_cancel_id_post

> models::TriggerV2OrdersDcaCancelIdPost200Response trigger_v2_orders_dca_cancel_id_post(id)
Initiate DCA cancellation

Step 1 of the two-step cancellation. Moves the order to `withdrawing` and returns an unsigned withdrawal transaction for the unfilled remainder. Already-filled rounds are not reversed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | DCA order UUID | [required] |

### Return type

[**models::TriggerV2OrdersDcaCancelIdPost200Response**](_trigger_v2_orders_dca_cancel__id__post_200_response.md)

### Authorization

[TriggerV2_ApiKeyAuth](../README.md#TriggerV2_ApiKeyAuth), [TriggerV2_BearerAuth](../README.md#TriggerV2_BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_v2_orders_dca_confirm_cancel_id_post

> models::TriggerV2TxSignatureResponse trigger_v2_orders_dca_confirm_cancel_id_post(id, trigger_v2_orders_dca_confirm_cancel_id_post_request)
Confirm DCA cancellation

Step 2 of the two-step cancellation. Submit the signed withdrawal transaction to return the remaining funds and move the order to `cancelled`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | DCA order UUID | [required] |
**trigger_v2_orders_dca_confirm_cancel_id_post_request** | [**TriggerV2OrdersDcaConfirmCancelIdPostRequest**](TriggerV2OrdersDcaConfirmCancelIdPostRequest.md) |  | [required] |

### Return type

[**models::TriggerV2TxSignatureResponse**](TriggerV2_TxSignatureResponse.md)

### Authorization

[TriggerV2_ApiKeyAuth](../README.md#TriggerV2_ApiKeyAuth), [TriggerV2_BearerAuth](../README.md#TriggerV2_BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_v2_orders_dca_post

> models::TriggerV2TxSignatureResponse trigger_v2_orders_dca_post(trigger_v2_orders_dca_post_request)
Create DCA order

Create a dollar-cost averaging order that splits a single deposit into `orderCount` equal rounds executed every `intervalSeconds`. Submit the signed deposit transaction from `/deposit/craft` (crafted with `orderType: \"dca\"`).  Call this path with no trailing slash. `POST /orders/dca/` returns 404.  Rounds execute as Jupiter swaps with Ultra-managed (RTSE) slippage; there is no slippage parameter. Each round pays the standard Jupiter swap fee (no separate DCA fee), and the output settles directly to the taker's wallet. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trigger_v2_orders_dca_post_request** | [**TriggerV2OrdersDcaPostRequest**](TriggerV2OrdersDcaPostRequest.md) |  | [required] |

### Return type

[**models::TriggerV2TxSignatureResponse**](TriggerV2_TxSignatureResponse.md)

### Authorization

[TriggerV2_ApiKeyAuth](../README.md#TriggerV2_ApiKeyAuth), [TriggerV2_BearerAuth](../README.md#TriggerV2_BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_v2_orders_history_dca_get

> models::TriggerV2OrdersHistoryDcaGet200Response trigger_v2_orders_history_dca_get(state, mint, limit, offset, sort, dir)
List DCA orders

List the authenticated wallet's DCA orders with full event history.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state** | Option<**String**> | active = depositing/active/executing/withdrawing; past = completed/cancelled/deposit_failed |  |
**mint** | Option<**String**> | Filter by mint (matches inputMint or outputMint) |  |
**limit** | Option<**f64**> |  |  |[default to 20]
**offset** | Option<**f64**> |  |  |[default to 0]
**sort** | Option<**String**> |  |  |[default to updated_at]
**dir** | Option<**String**> |  |  |[default to desc]

### Return type

[**models::TriggerV2OrdersHistoryDcaGet200Response**](_trigger_v2_orders_history_dca_get_200_response.md)

### Authorization

[TriggerV2_ApiKeyAuth](../README.md#TriggerV2_ApiKeyAuth), [TriggerV2_BearerAuth](../README.md#TriggerV2_BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_v2_orders_history_dca_id_get

> models::TriggerV2DcaHistoryItem trigger_v2_orders_history_dca_id_get(id)
Get DCA order

Get a single DCA order by ID, including fill history and events. The path requires the `dca` segment. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | DCA order UUID | [required] |

### Return type

[**models::TriggerV2DcaHistoryItem**](TriggerV2_DcaHistoryItem.md)

### Authorization

[TriggerV2_ApiKeyAuth](../README.md#TriggerV2_ApiKeyAuth), [TriggerV2_BearerAuth](../README.md#TriggerV2_BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_v2_orders_history_get

> models::TriggerV2OrdersHistoryGet200Response trigger_v2_orders_history_get(state, mint, limit, offset, sort, dir)
Get order history

Retrieve paginated order history for the authenticated wallet. Filter by state, mint, and sort order. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state** | Option<**String**> | Filter by order state group |  |
**mint** | Option<**String**> | Filter by token mint address |  |
**limit** | Option<**f64**> | Results per page |  |[default to 20]
**offset** | Option<**f64**> | Number of results to skip |  |[default to 0]
**sort** | Option<**String**> | Sort field |  |[default to updated_at]
**dir** | Option<**String**> | Sort direction |  |[default to desc]

### Return type

[**models::TriggerV2OrdersHistoryGet200Response**](_trigger_v2_orders_history_get_200_response.md)

### Authorization

[TriggerV2_ApiKeyAuth](../README.md#TriggerV2_ApiKeyAuth), [TriggerV2_BearerAuth](../README.md#TriggerV2_BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_v2_orders_price_cancel_order_id_post

> models::TriggerV2OrdersPriceCancelOrderIdPost200Response trigger_v2_orders_price_cancel_order_id_post(order_id)
Initiate order cancellation

Step 1 of the two-step cancellation flow. Returns a withdrawal transaction that must be signed and submitted via the confirm-cancel endpoint. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **String** |  | [required] |

### Return type

[**models::TriggerV2OrdersPriceCancelOrderIdPost200Response**](_trigger_v2_orders_price_cancel__orderId__post_200_response.md)

### Authorization

[TriggerV2_ApiKeyAuth](../README.md#TriggerV2_ApiKeyAuth), [TriggerV2_BearerAuth](../README.md#TriggerV2_BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_v2_orders_price_confirm_cancel_order_id_post

> models::TriggerV2TxSignatureResponse trigger_v2_orders_price_confirm_cancel_order_id_post(order_id, trigger_v2_orders_price_confirm_cancel_order_id_post_request)
Confirm order cancellation

Step 2 of the two-step cancellation flow. Submit the signed withdrawal transaction to complete the cancellation and return funds to your wallet. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **String** |  | [required] |
**trigger_v2_orders_price_confirm_cancel_order_id_post_request** | [**TriggerV2OrdersPriceConfirmCancelOrderIdPostRequest**](TriggerV2OrdersPriceConfirmCancelOrderIdPostRequest.md) |  | [required] |

### Return type

[**models::TriggerV2TxSignatureResponse**](TriggerV2_TxSignatureResponse.md)

### Authorization

[TriggerV2_ApiKeyAuth](../README.md#TriggerV2_ApiKeyAuth), [TriggerV2_BearerAuth](../README.md#TriggerV2_BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_v2_orders_price_order_id_patch

> models::TriggerV2OrdersPriceOrderIdPatch200Response trigger_v2_orders_price_order_id_patch(order_id, trigger_v2_orders_price_order_id_patch_request)
Update price order

Update trigger price or slippage of an existing order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **String** | Order UUID | [required] |
**trigger_v2_orders_price_order_id_patch_request** | [**TriggerV2OrdersPriceOrderIdPatchRequest**](TriggerV2OrdersPriceOrderIdPatchRequest.md) |  | [required] |

### Return type

[**models::TriggerV2OrdersPriceOrderIdPatch200Response**](_trigger_v2_orders_price__orderId__patch_200_response.md)

### Authorization

[TriggerV2_ApiKeyAuth](../README.md#TriggerV2_ApiKeyAuth), [TriggerV2_BearerAuth](../README.md#TriggerV2_BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_v2_orders_price_post

> models::TriggerV2OrderResponse trigger_v2_orders_price_post(trigger_v2_orders_price_post_request)
Create price order

Create a new price order. Supports three order types: - `single`: triggers when price crosses above/below threshold - `oco`: one-cancels-other (take-profit + stop-loss pair) - `otoco`: parent trigger activates a TP/SL pair on fill 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trigger_v2_orders_price_post_request** | [**TriggerV2OrdersPricePostRequest**](TriggerV2OrdersPricePostRequest.md) |  | [required] |

### Return type

[**models::TriggerV2OrderResponse**](TriggerV2_OrderResponse.md)

### Authorization

[TriggerV2_ApiKeyAuth](../README.md#TriggerV2_ApiKeyAuth), [TriggerV2_BearerAuth](../README.md#TriggerV2_BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_v2_vault_get

> models::TriggerV2VaultGet200Response trigger_v2_vault_get()
Get vault info

Retrieve the vault associated with your authenticated wallet.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::TriggerV2VaultGet200Response**](_trigger_v2_vault_get_200_response.md)

### Authorization

[TriggerV2_ApiKeyAuth](../README.md#TriggerV2_ApiKeyAuth), [TriggerV2_BearerAuth](../README.md#TriggerV2_BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_v2_vault_register_get

> models::TriggerV2VaultRegisterGet201Response trigger_v2_vault_register_get()
Register a new vault

Create a new Privy-managed vault wallet for your account. Call this once per wallet. Subsequent calls return the existing vault. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::TriggerV2VaultRegisterGet201Response**](_trigger_v2_vault_register_get_201_response.md)

### Authorization

[TriggerV2_ApiKeyAuth](../README.md#TriggerV2_ApiKeyAuth), [TriggerV2_BearerAuth](../README.md#TriggerV2_BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

