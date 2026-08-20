# \StudioApi

All URIs are relative to *https://api.jup.ag*

Method | HTTP request | Description
------------- | ------------- | -------------
[**studio_v1_dbc_fee_create_tx_post**](StudioApi.md#studio_v1_dbc_fee_create_tx_post) | **POST** /studio/v1/dbc/fee/create-tx | dbc-fee-create-tx
[**studio_v1_dbc_fee_post**](StudioApi.md#studio_v1_dbc_fee_post) | **POST** /studio/v1/dbc/fee | dbc-fee
[**studio_v1_dbc_pool_addresses_mint_get**](StudioApi.md#studio_v1_dbc_pool_addresses_mint_get) | **GET** /studio/v1/dbc-pool/addresses/{mint} | dbc-pool-addresses-by-mint
[**studio_v1_dbc_pool_create_tx_post**](StudioApi.md#studio_v1_dbc_pool_create_tx_post) | **POST** /studio/v1/dbc-pool/create-tx | dbc-pool-create-tx
[**studio_v1_dbc_pool_submit_post**](StudioApi.md#studio_v1_dbc_pool_submit_post) | **POST** /studio/v1/dbc-pool/submit | dbc-pool-submit



## studio_v1_dbc_fee_create_tx_post

> models::StudioV1DbcFeeCreateTxPost200Response studio_v1_dbc_fee_create_tx_post(studio_create_claim_fee_dbc_transaction_request_body)
dbc-fee-create-tx

Request for a base64-encoded unsigned transaction to claim creator trading fees of a Dynamic Bonding Curve pool - Handles both direct creator ownership and proxy-based ownership. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**studio_create_claim_fee_dbc_transaction_request_body** | Option<[**StudioCreateClaimFeeDbcTransactionRequestBody**](StudioCreateClaimFeeDbcTransactionRequestBody.md)> |  |  |

### Return type

[**models::StudioV1DbcFeeCreateTxPost200Response**](_studio_v1_dbc_fee_create_tx_post_200_response.md)

### Authorization

[Studio_ApiKeyAuth](../README.md#Studio_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## studio_v1_dbc_fee_post

> models::StudioV1DbcFeePost200Response studio_v1_dbc_fee_post(studio_v1_dbc_fee_post_request)
dbc-fee

Request for unclaimed creator trading fees of a Dynamic Bonding Curve pool 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**studio_v1_dbc_fee_post_request** | Option<[**StudioV1DbcFeePostRequest**](StudioV1DbcFeePostRequest.md)> |  |  |

### Return type

[**models::StudioV1DbcFeePost200Response**](_studio_v1_dbc_fee_post_200_response.md)

### Authorization

[Studio_ApiKeyAuth](../README.md#Studio_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## studio_v1_dbc_pool_addresses_mint_get

> models::StudioV1DbcPoolAddressesMintGet200Response studio_v1_dbc_pool_addresses_mint_get(mint)
dbc-pool-addresses-by-mint

Request for pool addresses for a given token mint 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mint** | **String** |  | [required] |

### Return type

[**models::StudioV1DbcPoolAddressesMintGet200Response**](_studio_v1_dbc_pool_addresses__mint__get_200_response.md)

### Authorization

[Studio_ApiKeyAuth](../README.md#Studio_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## studio_v1_dbc_pool_create_tx_post

> models::StudioCreateDbcTransactionResponse studio_v1_dbc_pool_create_tx_post(studio_create_dbc_transaction_request_body)
dbc-pool-create-tx

Request for a base64-encoded unsigned transaction to create a Dynamic Bonding Curve pool with token metadata 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**studio_create_dbc_transaction_request_body** | Option<[**StudioCreateDbcTransactionRequestBody**](StudioCreateDbcTransactionRequestBody.md)> |  |  |

### Return type

[**models::StudioCreateDbcTransactionResponse**](Studio_CreateDBCTransactionResponse.md)

### Authorization

[Studio_ApiKeyAuth](../README.md#Studio_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## studio_v1_dbc_pool_submit_post

> models::StudioV1DbcPoolSubmitPost200Response studio_v1_dbc_pool_submit_post(transaction, owner, content, header_image)
dbc-pool-submit

Execute the signed transaction, and optionally upload content and header image 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction** | **String** | - Base64-encoded signed transaction  | [required] |
**owner** | **String** | - Owner wallet public key  | [required] |
**content** | Option<**String**> | - Optional content description displayed on Studio dedicated token page - This is stored off-chain for our frontend to display - This is NOT the on-chain token metadata - The on-chain token metadata is done by making a PUT request to the presigned URL  |  |
**header_image** | Option<**std::path::PathBuf**> | - Optional header image file displayed on Studio token page - This is stored off-chain for our frontend to display - This is NOT the on-chain token metadata - The on-chain token metadata is done by making a PUT request to the presigned URL  |  |

### Return type

[**models::StudioV1DbcPoolSubmitPost200Response**](_studio_v1_dbc_pool_submit_post_200_response.md)

### Authorization

[Studio_ApiKeyAuth](../README.md#Studio_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

