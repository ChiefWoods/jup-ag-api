# \TokensV1Api

All URIs are relative to *https://api.jup.ag*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tokens_v1_all_get**](TokensV1Api.md#tokens_v1_all_get) | **GET** /tokens/v1/all | all
[**tokens_v1_market_market_address_mints_get**](TokensV1Api.md#tokens_v1_market_market_address_mints_get) | **GET** /tokens/v1/market/{market_address}/mints | mints in market
[**tokens_v1_mints_tradable_get**](TokensV1Api.md#tokens_v1_mints_tradable_get) | **GET** /tokens/v1/mints/tradable | tradable
[**tokens_v1_new_get**](TokensV1Api.md#tokens_v1_new_get) | **GET** /tokens/v1/new | new
[**tokens_v1_tagged_tag_get**](TokensV1Api.md#tokens_v1_tagged_tag_get) | **GET** /tokens/v1/tagged/{tag} | tagged
[**tokens_v1_token_mint_address_get**](TokensV1Api.md#tokens_v1_token_mint_address_get) | **GET** /tokens/v1/token/{mint_address} | token information



## tokens_v1_all_get

> Vec<models::TokensV1Mint> tokens_v1_all_get()
all

Returns all tokens with all metadata. Tokens API V1 is deprecated, please use Tokens API V2 instead. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::TokensV1Mint>**](TokensV1_Mint.md)

### Authorization

[TokensV1_ApiKeyAuth](../README.md#TokensV1_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tokens_v1_market_market_address_mints_get

> Vec<String> tokens_v1_market_market_address_mints_get(market_address)
mints in market

Returns the mints involved in a market. Tokens API V1 is deprecated, please use Tokens API V2 instead. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**market_address** | **String** | A market/pool address | [required] |

### Return type

**Vec<String>**

### Authorization

[TokensV1_ApiKeyAuth](../README.md#TokensV1_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tokens_v1_mints_tradable_get

> Vec<String> tokens_v1_mints_tradable_get()
tradable

Returns a list of all mints tradable via Jupiter routing. Tokens API V1 is deprecated, please use Tokens API V2 instead. 

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[TokensV1_ApiKeyAuth](../README.md#TokensV1_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tokens_v1_new_get

> Vec<models::TokensV1MintWithCreationTimeAndMarkets> tokens_v1_new_get(limit, offset)
new

Returns new tokens with metadata, created at timestamp and markets. Tokens API V1 is deprecated, please use Tokens API V2 instead. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | How many records to output in the result |  |
**offset** | Option<**i64**> | The offset into the result set. Used in conjunction with `limit` to page through the data. |  |[default to 0]

### Return type

[**Vec<models::TokensV1MintWithCreationTimeAndMarkets>**](TokensV1_MintWithCreationTimeAndMarkets.md)

### Authorization

[TokensV1_ApiKeyAuth](../README.md#TokensV1_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tokens_v1_tagged_tag_get

> models::TokensV1MintIncludingDuplicates tokens_v1_tagged_tag_get(tag)
tagged

Returns a list of mints with specified tag(s) along with their metadata. Tokens API V1 is deprecated, please use Tokens API V2 instead. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag** | **String** | A list of one or more tags, comma separated. The list is the union of tokens with these tags. | [required] |

### Return type

[**models::TokensV1MintIncludingDuplicates**](TokensV1_MintIncludingDuplicates.md)

### Authorization

[TokensV1_ApiKeyAuth](../README.md#TokensV1_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tokens_v1_token_mint_address_get

> models::TokensV1MintIncludingDuplicates tokens_v1_token_mint_address_get(mint_address)
token information

Returns the specified mint address's token information and metadata. Tokens API V1 is deprecated, please use Tokens API V2 instead. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mint_address** | **String** | The token's mint address | [required] |

### Return type

[**models::TokensV1MintIncludingDuplicates**](TokensV1_MintIncludingDuplicates.md)

### Authorization

[TokensV1_ApiKeyAuth](../README.md#TokensV1_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

