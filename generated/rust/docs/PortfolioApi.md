# \PortfolioApi

All URIs are relative to *https://api.jup.ag*

Method | HTTP request | Description
------------- | ------------- | -------------
[**portfolio_v1_platforms_get**](PortfolioApi.md#portfolio_v1_platforms_get) | **GET** /portfolio/v1/platforms | 
[**portfolio_v1_positions_address_get**](PortfolioApi.md#portfolio_v1_positions_address_get) | **GET** /portfolio/v1/positions/{address} | 
[**portfolio_v1_staked_jup_address_get**](PortfolioApi.md#portfolio_v1_staked_jup_address_get) | **GET** /portfolio/v1/staked-jup/{address} | 



## portfolio_v1_platforms_get

> Vec<models::PortfolioV1PlatformsGet200ResponseInner> portfolio_v1_platforms_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::PortfolioV1PlatformsGet200ResponseInner>**](_portfolio_v1_platforms_get_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolio_v1_positions_address_get

> models::PortfolioV1PositionsAddressGet200Response portfolio_v1_positions_address_get(address, platforms)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | Solana wallet address | [required] |
**platforms** | Option<**String**> | Comma separated list of platform ids |  |

### Return type

[**models::PortfolioV1PositionsAddressGet200Response**](_portfolio_v1_positions__address__get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolio_v1_staked_jup_address_get

> models::PortfolioV1StakedJupAddressGet200Response portfolio_v1_staked_jup_address_get(address)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |

### Return type

[**models::PortfolioV1StakedJupAddressGet200Response**](_portfolio_v1_staked_jup__address__get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

