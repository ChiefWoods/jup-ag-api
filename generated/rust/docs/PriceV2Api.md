# \PriceV2Api

All URIs are relative to *https://api.jup.ag*

Method | HTTP request | Description
------------- | ------------- | -------------
[**price_v2_get**](PriceV2Api.md#price_v2_get) | **GET** /price/v2/ | price



## price_v2_get

> models::PriceV2PriceResponse price_v2_get(ids, vs_token, show_extra_info)
price

Returns prices of specified tokens. Price V2 API is deprecated, please use Price V3 API instead. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | Comma separate to pass in multiple | [required] |
**vs_token** | Option<**String**> | By default, prices are denominated by USD. To denominate price in SOL, use `vsToken` with SOL mint address |  |
**show_extra_info** | Option<**String**> | To use, pass in `showExtraInfo=true`, cannot use `vsToken` with this parameter |  |

### Return type

[**models::PriceV2PriceResponse**](PriceV2_PriceResponse.md)

### Authorization

[PriceV2_ApiKeyAuth](../README.md#PriceV2_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

