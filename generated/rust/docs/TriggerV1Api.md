# \TriggerV1Api

All URIs are relative to *https://api.jup.ag*

Method | HTTP request | Description
------------- | ------------- | -------------
[**trigger_v1_cancel_order_post**](TriggerV1Api.md#trigger_v1_cancel_order_post) | **POST** /trigger/v1/cancelOrder | cancelOrder
[**trigger_v1_cancel_orders_post**](TriggerV1Api.md#trigger_v1_cancel_orders_post) | **POST** /trigger/v1/cancelOrders | cancelOrders
[**trigger_v1_create_order_post**](TriggerV1Api.md#trigger_v1_create_order_post) | **POST** /trigger/v1/createOrder | createOrder
[**trigger_v1_execute_post**](TriggerV1Api.md#trigger_v1_execute_post) | **POST** /trigger/v1/execute | execute
[**trigger_v1_get_trigger_orders_get**](TriggerV1Api.md#trigger_v1_get_trigger_orders_get) | **GET** /trigger/v1/getTriggerOrders | getTriggerOrders



## trigger_v1_cancel_order_post

> models::TriggerV1CancelOrderPost200Response trigger_v1_cancel_order_post(trigger_v1_cancel_order_post_request)
cancelOrder

Request for a base64-encoded unsigned trigger order cancellation transaction to be used in `POST /trigger/v1/execute` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trigger_v1_cancel_order_post_request** | Option<[**TriggerV1CancelOrderPostRequest**](TriggerV1CancelOrderPostRequest.md)> |  |  |

### Return type

[**models::TriggerV1CancelOrderPost200Response**](_trigger_v1_cancelOrder_post_200_response.md)

### Authorization

[TriggerV1_ApiKeyAuth](../README.md#TriggerV1_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_v1_cancel_orders_post

> models::TriggerV1CancelOrdersPost200Response trigger_v1_cancel_orders_post(cancel_orders_request_body)
cancelOrders

Request for a base64-encoded unsigned trigger order cancellation transaction(s) to be used in `POST /trigger/v1/execute` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cancel_orders_request_body** | Option<[**CancelOrdersRequestBody**](CancelOrdersRequestBody.md)> |  |  |

### Return type

[**models::TriggerV1CancelOrdersPost200Response**](_trigger_v1_cancelOrders_post_200_response.md)

### Authorization

[TriggerV1_ApiKeyAuth](../README.md#TriggerV1_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_v1_create_order_post

> models::TriggerV1CreateOrderPost200Response trigger_v1_create_order_post(create_orders_request_body)
createOrder

Request for a base64-encoded unsigned trigger order creation transaction to be used in `POST /trigger/v1/execute` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_orders_request_body** | Option<[**CreateOrdersRequestBody**](CreateOrdersRequestBody.md)> |  |  |

### Return type

[**models::TriggerV1CreateOrderPost200Response**](_trigger_v1_createOrder_post_200_response.md)

### Authorization

[TriggerV1_ApiKeyAuth](../README.md#TriggerV1_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_v1_execute_post

> models::TriggerV1ExecutePost200Response trigger_v1_execute_post(execute_request_body)
execute

Execute the signed transaction and get the execution status 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**execute_request_body** | [**ExecuteRequestBody**](ExecuteRequestBody.md) |  | [required] |

### Return type

[**models::TriggerV1ExecutePost200Response**](_trigger_v1_execute_post_200_response.md)

### Authorization

[TriggerV1_ApiKeyAuth](../README.md#TriggerV1_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_v1_get_trigger_orders_get

> models::TriggerV1GetTriggerOrdersGet200Response trigger_v1_get_trigger_orders_get(user, order_status, page, include_failed_tx, input_mint, output_mint)
getTriggerOrders

Request for the active or historical orders associated to the provided account 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user** | **String** |  | [required] |
**order_status** | **String** | The status of the orders to return | [required] |
**page** | Option<**String**> |  |  |[default to 1]
**include_failed_tx** | Option<**String**> | Whether to include failed transactions, expects 'true' or 'false' |  |
**input_mint** | Option<**String**> | The input mint to filter by |  |
**output_mint** | Option<**String**> | The output mint to filter by |  |

### Return type

[**models::TriggerV1GetTriggerOrdersGet200Response**](_trigger_v1_getTriggerOrders_get_200_response.md)

### Authorization

[TriggerV1_ApiKeyAuth](../README.md#TriggerV1_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

