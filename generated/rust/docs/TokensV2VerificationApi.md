# \TokensV2VerificationApi

All URIs are relative to *https://api.jup.ag*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tokens_v2_verify_express_check_eligibility_get**](TokensV2VerificationApi.md#tokens_v2_verify_express_check_eligibility_get) | **GET** /tokens/v2/verify/express/check-eligibility | Check express verification eligibility
[**tokens_v2_verify_express_craft_txn_get**](TokensV2VerificationApi.md#tokens_v2_verify_express_craft_txn_get) | **GET** /tokens/v2/verify/express/craft-txn | Craft express verification payment transaction
[**tokens_v2_verify_express_execute_post**](TokensV2VerificationApi.md#tokens_v2_verify_express_execute_post) | **POST** /tokens/v2/verify/express/execute | Execute express verification



## tokens_v2_verify_express_check_eligibility_get

> models::TokensV2VerificationCheckEligibilityResponse tokens_v2_verify_express_check_eligibility_get(token_id)
Check express verification eligibility

Checks whether a token is eligible for express verification and metadata updates. If both `canVerify` and `canMetadata` are `false`, any submission will be rejected before payment is charged. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_id** | **String** | Token mint address to check eligibility for | [required] |

### Return type

[**models::TokensV2VerificationCheckEligibilityResponse**](TokensV2Verification_CheckEligibilityResponse.md)

### Authorization

[TokensV2Verification_ApiKeyAuth](../README.md#TokensV2Verification_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tokens_v2_verify_express_craft_txn_get

> models::TokensV2VerificationCraftTxnResponse tokens_v2_verify_express_craft_txn_get(sender_address, payment_currency)
Craft express verification payment transaction

Returns an unsigned Solana transaction worth 1000 JUP as payment for express verification. JUP transfers directly; SOL, USDC, and JUPUSD craft an Ultra swap to 1000 JUP. The `requestId` in the response is required for the execute step. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sender_address** | **String** | Solana wallet address that will sign and pay for the transaction | [required] |
**payment_currency** | Option<[**TokensV2VerificationPaymentCurrency**](TokensV2VerificationPaymentCurrency.md)> | Payment currency. Defaults to JUP (direct transfer). SOL/USDC/JUPUSD route through an Ultra swap to JUP. |  |

### Return type

[**models::TokensV2VerificationCraftTxnResponse**](TokensV2Verification_CraftTxnResponse.md)

### Authorization

[TokensV2Verification_ApiKeyAuth](../README.md#TokensV2Verification_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tokens_v2_verify_express_execute_post

> models::TokensV2VerificationExpressExecuteResponse tokens_v2_verify_express_execute_post(tokens_v2_verification_express_execute_body)
Execute express verification

Submits the signed payment transaction, records the payment, creates a verification request, and optionally submits token metadata updates. Verification and metadata updates are reviewed independently. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tokens_v2_verification_express_execute_body** | [**TokensV2VerificationExpressExecuteBody**](TokensV2VerificationExpressExecuteBody.md) |  | [required] |

### Return type

[**models::TokensV2VerificationExpressExecuteResponse**](TokensV2Verification_ExpressExecuteResponse.md)

### Authorization

[TokensV2Verification_ApiKeyAuth](../README.md#TokensV2Verification_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

