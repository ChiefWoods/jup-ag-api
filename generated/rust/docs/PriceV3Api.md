# \PriceV3Api

All URIs are relative to *https://api.jup.ag*

Method | HTTP request | Description
------------- | ------------- | -------------
[**price_v3_get**](PriceV3Api.md#price_v3_get) | **GET** /price/v3 | price



## price_v3_get

> std::collections::HashMap<String, models::PriceV3Get200ResponseValue> price_v3_get(ids)
price

Get USD prices for up to 50 tokens per request. Tokens without a reliable price are omitted from the response. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | Comma separate to pass in multiple | [required] |

### Return type

[**std::collections::HashMap<String, models::PriceV3Get200ResponseValue>**](_price_v3_get_200_response_value.md)

### Authorization

[PriceV3_ApiKeyAuth](../README.md#PriceV3_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

