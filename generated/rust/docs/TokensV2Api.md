# \TokensV2Api

All URIs are relative to *https://api.jup.ag*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tokens_v2_category_interval_get**](TokensV2Api.md#tokens_v2_category_interval_get) | **GET** /tokens/v2/{category}/{interval} | category
[**tokens_v2_recent_get**](TokensV2Api.md#tokens_v2_recent_get) | **GET** /tokens/v2/recent | recent
[**tokens_v2_search_get**](TokensV2Api.md#tokens_v2_search_get) | **GET** /tokens/v2/search | search
[**tokens_v2_tag_get**](TokensV2Api.md#tokens_v2_tag_get) | **GET** /tokens/v2/tag | tag



## tokens_v2_category_interval_get

> Vec<models::TokensV2MintInformation> tokens_v2_category_interval_get(category, interval, limit)
category

Returns an array of mints and their information for the given category and interval 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category** | **String** | - Top tokens in different trading categories - The result filters out generic top tokens like SOL, USDC, etc - Default to 50 mints in response  | [required] |
**interval** | **String** | - Query by time interval for more accuracy  | [required] |
**limit** | Option<**i32**> | - Maximum number of results to return (default 50, max 100)  |  |

### Return type

[**Vec<models::TokensV2MintInformation>**](TokensV2_MintInformation.md)

### Authorization

[TokensV2_ApiKeyAuth](../README.md#TokensV2_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tokens_v2_recent_get

> Vec<models::TokensV2MintInformation> tokens_v2_recent_get()
recent

Returns an array of mints that recently had their **first created pool** - Default to 30 mints in response 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::TokensV2MintInformation>**](TokensV2_MintInformation.md)

### Authorization

[TokensV2_ApiKeyAuth](../README.md#TokensV2_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tokens_v2_search_get

> Vec<models::TokensV2MintInformation> tokens_v2_search_get(query)
search

Request a search by token's symbol, name or mint address 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | - Search for a token and its information by its symbol, name or mint address - Comma-separate to ONLY search for multiple mint addresses - Limit to 100 mint addresses in query - Default to 20 mints in response when searching via symbol or name  | [required] |

### Return type

[**Vec<models::TokensV2MintInformation>**](TokensV2_MintInformation.md)

### Authorization

[TokensV2_ApiKeyAuth](../README.md#TokensV2_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tokens_v2_tag_get

> Vec<models::TokensV2MintInformation> tokens_v2_tag_get(query)
tag

Request an array of mints and their information by a tag - Note that this will return the entire array of existing mints that belongs to the tag. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** |  | [required] |

### Return type

[**Vec<models::TokensV2MintInformation>**](TokensV2_MintInformation.md)

### Authorization

[TokensV2_ApiKeyAuth](../README.md#TokensV2_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

