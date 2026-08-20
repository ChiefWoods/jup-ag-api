# \LendApi

All URIs are relative to *https://api.jup.ag*

Method | HTTP request | Description
------------- | ------------- | -------------
[**lend_v1_earn_deposit_instructions_post**](LendApi.md#lend_v1_earn_deposit_instructions_post) | **POST** /lend/v1/earn/deposit-instructions | deposit-instructions
[**lend_v1_earn_deposit_post**](LendApi.md#lend_v1_earn_deposit_post) | **POST** /lend/v1/earn/deposit | deposit
[**lend_v1_earn_earnings_get**](LendApi.md#lend_v1_earn_earnings_get) | **GET** /lend/v1/earn/earnings | earnings
[**lend_v1_earn_mint_instructions_post**](LendApi.md#lend_v1_earn_mint_instructions_post) | **POST** /lend/v1/earn/mint-instructions | mint-instructions
[**lend_v1_earn_mint_post**](LendApi.md#lend_v1_earn_mint_post) | **POST** /lend/v1/earn/mint | mint
[**lend_v1_earn_positions_get**](LendApi.md#lend_v1_earn_positions_get) | **GET** /lend/v1/earn/positions | positions
[**lend_v1_earn_redeem_instructions_post**](LendApi.md#lend_v1_earn_redeem_instructions_post) | **POST** /lend/v1/earn/redeem-instructions | redeem-instructions
[**lend_v1_earn_redeem_post**](LendApi.md#lend_v1_earn_redeem_post) | **POST** /lend/v1/earn/redeem | redeem
[**lend_v1_earn_tokens_get**](LendApi.md#lend_v1_earn_tokens_get) | **GET** /lend/v1/earn/tokens | tokens
[**lend_v1_earn_withdraw_instructions_post**](LendApi.md#lend_v1_earn_withdraw_instructions_post) | **POST** /lend/v1/earn/withdraw-instructions | withdraw-instructions
[**lend_v1_earn_withdraw_post**](LendApi.md#lend_v1_earn_withdraw_post) | **POST** /lend/v1/earn/withdraw | withdraw



## lend_v1_earn_deposit_instructions_post

> models::LendInstructionResponse lend_v1_earn_deposit_instructions_post(lend_earn_amount_request_body)
deposit-instructions

Request for the instruction of an earn deposit transaction to deposit assets 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lend_earn_amount_request_body** | [**LendEarnAmountRequestBody**](LendEarnAmountRequestBody.md) |  | [required] |

### Return type

[**models::LendInstructionResponse**](Lend_InstructionResponse.md)

### Authorization

[Lend_ApiKeyAuth](../README.md#Lend_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lend_v1_earn_deposit_post

> models::LendTransactionResponse lend_v1_earn_deposit_post(lend_earn_amount_request_body)
deposit

Request for a base64-encoded unsigned earn deposit transaction to deposit assets 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lend_earn_amount_request_body** | [**LendEarnAmountRequestBody**](LendEarnAmountRequestBody.md) |  | [required] |

### Return type

[**models::LendTransactionResponse**](Lend_TransactionResponse.md)

### Authorization

[Lend_ApiKeyAuth](../README.md#Lend_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lend_v1_earn_earnings_get

> models::LendUserEarningsResponse lend_v1_earn_earnings_get(user, positions)
earnings

Request for the earnings of one or multiple positions of a user 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user** | **String** | User wallet address  | [required] |
**positions** | **String** | User token positions (comma-separated)  | [required] |

### Return type

[**models::LendUserEarningsResponse**](Lend_UserEarningsResponse.md)

### Authorization

[Lend_ApiKeyAuth](../README.md#Lend_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lend_v1_earn_mint_instructions_post

> models::LendInstructionResponse lend_v1_earn_mint_instructions_post(lend_earn_shares_request_body)
mint-instructions

Request for the instruction of an earn mint transaction to mint shares 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lend_earn_shares_request_body** | [**LendEarnSharesRequestBody**](LendEarnSharesRequestBody.md) |  | [required] |

### Return type

[**models::LendInstructionResponse**](Lend_InstructionResponse.md)

### Authorization

[Lend_ApiKeyAuth](../README.md#Lend_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lend_v1_earn_mint_post

> models::LendTransactionResponse lend_v1_earn_mint_post(lend_earn_shares_request_body)
mint

Request for a base64-encoded unsigned earn mint transaction to mint shares 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lend_earn_shares_request_body** | [**LendEarnSharesRequestBody**](LendEarnSharesRequestBody.md) |  | [required] |

### Return type

[**models::LendTransactionResponse**](Lend_TransactionResponse.md)

### Authorization

[Lend_ApiKeyAuth](../README.md#Lend_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lend_v1_earn_positions_get

> Vec<models::LendUserPosition> lend_v1_earn_positions_get(users)
positions

Request for the position data of one or multiple users 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**users** | **String** | User wallet addresses (comma separated)  | [required] |

### Return type

[**Vec<models::LendUserPosition>**](Lend_UserPosition.md)

### Authorization

[Lend_ApiKeyAuth](../README.md#Lend_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lend_v1_earn_redeem_instructions_post

> models::LendInstructionResponse lend_v1_earn_redeem_instructions_post(lend_earn_shares_request_body)
redeem-instructions

Request for the instruction of an earn redeem transaction to redeem shares 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lend_earn_shares_request_body** | [**LendEarnSharesRequestBody**](LendEarnSharesRequestBody.md) |  | [required] |

### Return type

[**models::LendInstructionResponse**](Lend_InstructionResponse.md)

### Authorization

[Lend_ApiKeyAuth](../README.md#Lend_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lend_v1_earn_redeem_post

> models::LendTransactionResponse lend_v1_earn_redeem_post(lend_earn_shares_request_body)
redeem

Request for a base64-encoded unsigned earn redeem transaction to redeem shares 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lend_earn_shares_request_body** | [**LendEarnSharesRequestBody**](LendEarnSharesRequestBody.md) |  | [required] |

### Return type

[**models::LendTransactionResponse**](Lend_TransactionResponse.md)

### Authorization

[Lend_ApiKeyAuth](../README.md#Lend_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lend_v1_earn_tokens_get

> Vec<models::LendTokenInfo> lend_v1_earn_tokens_get()
tokens

Request for the tokens available to be deposited and their information 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::LendTokenInfo>**](Lend_TokenInfo.md)

### Authorization

[Lend_ApiKeyAuth](../README.md#Lend_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lend_v1_earn_withdraw_instructions_post

> models::LendInstructionResponse lend_v1_earn_withdraw_instructions_post(lend_earn_amount_request_body)
withdraw-instructions

Request for the instruction of an earn withdraw transaction to withdraw assets 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lend_earn_amount_request_body** | [**LendEarnAmountRequestBody**](LendEarnAmountRequestBody.md) |  | [required] |

### Return type

[**models::LendInstructionResponse**](Lend_InstructionResponse.md)

### Authorization

[Lend_ApiKeyAuth](../README.md#Lend_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lend_v1_earn_withdraw_post

> models::LendTransactionResponse lend_v1_earn_withdraw_post(lend_earn_amount_request_body)
withdraw

Request for a base64-encoded unsigned earn withdraw transaction to withdraw assets 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lend_earn_amount_request_body** | [**LendEarnAmountRequestBody**](LendEarnAmountRequestBody.md) |  | [required] |

### Return type

[**models::LendTransactionResponse**](Lend_TransactionResponse.md)

### Authorization

[Lend_ApiKeyAuth](../README.md#Lend_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

