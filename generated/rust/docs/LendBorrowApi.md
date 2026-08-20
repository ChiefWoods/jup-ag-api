# \LendBorrowApi

All URIs are relative to *https://api.jup.ag*

Method | HTTP request | Description
------------- | ------------- | -------------
[**build_borrow_operate_instructions**](LendBorrowApi.md#build_borrow_operate_instructions) | **POST** /lend/v1/borrow/operate-instructions | Build operate instructions
[**build_borrow_operate_transaction**](LendBorrowApi.md#build_borrow_operate_transaction) | **POST** /lend/v1/borrow/operate | Build unsigned operate transaction
[**list_borrow_positions**](LendBorrowApi.md#list_borrow_positions) | **GET** /lend/v1/borrow/positions | List borrow positions for users
[**list_borrow_vaults**](LendBorrowApi.md#list_borrow_vaults) | **GET** /lend/v1/borrow/vaults | List borrow vaults



## build_borrow_operate_instructions

> models::LendBorrowOperateInstructionsResponse build_borrow_operate_instructions(lend_borrow_operate_payload, market)
Build operate instructions

Returns Solana instructions and lookup table addresses for a borrow operate action.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lend_borrow_operate_payload** | [**LendBorrowOperatePayload**](LendBorrowOperatePayload.md) |  | [required] |
**market** | Option<[**LendBorrowMarket**](LendBorrowMarket.md)> | Lend market identifier. Defaults to `main`. |  |

### Return type

[**models::LendBorrowOperateInstructionsResponse**](LendBorrow_OperateInstructionsResponse.md)

### Authorization

[LendBorrow_ApiKeyAuth](../README.md#LendBorrow_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## build_borrow_operate_transaction

> models::LendBorrowOperateTransactionResponse build_borrow_operate_transaction(lend_borrow_operate_payload, market)
Build unsigned operate transaction

Builds a base64-encoded unsigned versioned transaction for a borrow operate action.  Positive `colAmount` / `debtAmount` values deposit collateral or borrow debt. Negative values withdraw collateral or pay back debt. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lend_borrow_operate_payload** | [**LendBorrowOperatePayload**](LendBorrowOperatePayload.md) |  | [required] |
**market** | Option<[**LendBorrowMarket**](LendBorrowMarket.md)> | Lend market identifier. Defaults to `main`. |  |

### Return type

[**models::LendBorrowOperateTransactionResponse**](LendBorrow_OperateTransactionResponse.md)

### Authorization

[LendBorrow_ApiKeyAuth](../README.md#LendBorrow_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_borrow_positions

> Vec<models::LendBorrowBorrowPosition> list_borrow_positions(users, market)
List borrow positions for users

Returns borrow positions (NFTs) for one or more wallet addresses.  Pass multiple wallets as a comma-separated list in the `users` query parameter. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**users** | **String** | Comma-separated Solana wallet addresses. | [required] |
**market** | Option<[**LendBorrowMarket**](LendBorrowMarket.md)> | Lend market identifier. Defaults to `main`. |  |

### Return type

[**Vec<models::LendBorrowBorrowPosition>**](LendBorrow_BorrowPosition.md)

### Authorization

[LendBorrow_ApiKeyAuth](../README.md#LendBorrow_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_borrow_vaults

> Vec<models::LendBorrowBorrowVault> list_borrow_vaults(market, rpc_url)
List borrow vaults

Returns all whitelisted borrow vaults for the requested market.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**market** | Option<[**LendBorrowMarket**](LendBorrowMarket.md)> | Lend market identifier. Defaults to `main`. |  |
**rpc_url** | Option<**String**> | Optional custom Solana RPC URL used to resolve on-chain vault state. |  |

### Return type

[**Vec<models::LendBorrowBorrowVault>**](LendBorrow_BorrowVault.md)

### Authorization

[LendBorrow_ApiKeyAuth](../README.md#LendBorrow_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

