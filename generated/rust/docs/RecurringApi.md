# \RecurringApi

All URIs are relative to *https://api.jup.ag*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_order**](RecurringApi.md#cancel_order) | **POST** /recurring/v1/cancelOrder | cancelOrder
[**create_order**](RecurringApi.md#create_order) | **POST** /recurring/v1/createOrder | createOrder
[**execute**](RecurringApi.md#execute) | **POST** /recurring/v1/execute | execute
[**price_deposit**](RecurringApi.md#price_deposit) | **POST** /recurring/v1/priceDeposit | priceDeposit
[**price_withdraw**](RecurringApi.md#price_withdraw) | **POST** /recurring/v1/priceWithdraw | priceWithdraw
[**recurring_v1_get_recurring_orders_get**](RecurringApi.md#recurring_v1_get_recurring_orders_get) | **GET** /recurring/v1/getRecurringOrders | getRecurringOrders



## cancel_order

> models::RecurringRecurringResponse cancel_order(recurring_close_recurring)
cancelOrder

Request for a base64-encoded unsigned recurring order cancellation transaction to be used in `POST /recurring/v1/execute` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recurring_close_recurring** | [**RecurringCloseRecurring**](RecurringCloseRecurring.md) |  | [required] |

### Return type

[**models::RecurringRecurringResponse**](Recurring_RecurringResponse.md)

### Authorization

[Recurring_ApiKeyAuth](../README.md#Recurring_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_order

> models::RecurringRecurringResponse create_order(recurring_create_recurring)
createOrder

Request for a base64-encoded unsigned recurring order creation transaction to be used in `POST /recurring/v1/execute` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recurring_create_recurring** | [**RecurringCreateRecurring**](RecurringCreateRecurring.md) |  | [required] |

### Return type

[**models::RecurringRecurringResponse**](Recurring_RecurringResponse.md)

### Authorization

[Recurring_ApiKeyAuth](../README.md#Recurring_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## execute

> models::RecurringExecuteRecurringResponse execute(recurring_execute_recurring)
execute

Execute the signed transaction and get the execution status 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recurring_execute_recurring** | [**RecurringExecuteRecurring**](RecurringExecuteRecurring.md) |  | [required] |

### Return type

[**models::RecurringExecuteRecurringResponse**](Recurring_ExecuteRecurringResponse.md)

### Authorization

[Recurring_ApiKeyAuth](../README.md#Recurring_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## price_deposit

> models::RecurringRecurringResponse price_deposit(recurring_deposit_price_recurring)
priceDeposit

Request for a base64-encoded unsigned price-based recurring order deposit transaction to be used in `POST /recurring/v1/execute`. Price-based recurring orders are deprecated, please use time-based recurring orders instead. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recurring_deposit_price_recurring** | [**RecurringDepositPriceRecurring**](RecurringDepositPriceRecurring.md) |  | [required] |

### Return type

[**models::RecurringRecurringResponse**](Recurring_RecurringResponse.md)

### Authorization

[Recurring_ApiKeyAuth](../README.md#Recurring_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## price_withdraw

> models::RecurringRecurringResponse price_withdraw(recurring_withdraw_price_recurring)
priceWithdraw

Request for a base64-encoded unsigned price-based recurring order withdrawal transaction to be used in `POST /recurring/v1/execute`. Price-based recurring orders are deprecated, please use time-based recurring orders instead. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recurring_withdraw_price_recurring** | [**RecurringWithdrawPriceRecurring**](RecurringWithdrawPriceRecurring.md) |  | [required] |

### Return type

[**models::RecurringRecurringResponse**](Recurring_RecurringResponse.md)

### Authorization

[Recurring_ApiKeyAuth](../README.md#Recurring_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## recurring_v1_get_recurring_orders_get

> models::RecurringGetRecurringOrderResponse recurring_v1_get_recurring_orders_get(recurring_type, order_status, user, page, mint, include_failed_tx)
getRecurringOrders

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recurring_type** | [**RecurringRecurringOrderType**](RecurringRecurringOrderType.md) |  | [required] |
**order_status** | [**RecurringOrderState**](RecurringOrderState.md) |  | [required] |
**user** | **String** |  | [required] |
**page** | Option<**i64**> |  | [required] |
**mint** | Option<**String**> |  | [required] |
**include_failed_tx** | **bool** |  | [required] |

### Return type

[**models::RecurringGetRecurringOrderResponse**](Recurring_GetRecurringOrderResponse.md)

### Authorization

[Recurring_ApiKeyAuth](../README.md#Recurring_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

