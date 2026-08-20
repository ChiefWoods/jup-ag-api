# \SwapV1Api

All URIs are relative to *https://api.jup.ag*

Method | HTTP request | Description
------------- | ------------- | -------------
[**program_id_to_label_get**](SwapV1Api.md#program_id_to_label_get) | **GET** /swap/v1/program-id-to-label | program-id-to-label
[**quote_get**](SwapV1Api.md#quote_get) | **GET** /swap/v1/quote | quote
[**swap_instructions_post**](SwapV1Api.md#swap_instructions_post) | **POST** /swap/v1/swap-instructions | swap-instructions
[**swap_post**](SwapV1Api.md#swap_post) | **POST** /swap/v1/swap | swap



## program_id_to_label_get

> std::collections::HashMap<String, String> program_id_to_label_get()
program-id-to-label

Returns a hash, which key is the program id and value is the label. This is used to help map error from transaction by identifying the fault program id. This can be used in conjunction with the `excludeDexes` or `dexes` parameter. 

### Parameters

This endpoint does not need any parameter.

### Return type

**std::collections::HashMap<String, String>**

### Authorization

[SwapV1_ApiKeyAuth](../README.md#SwapV1_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## quote_get

> models::SwapV1QuoteResponse quote_get(input_mint, output_mint, amount, slippage_bps, swap_mode, dexes, exclude_dexes, restrict_intermediate_tokens, only_direct_routes, as_legacy_transaction, platform_fee_bps, max_accounts, instruction_version, dynamic_slippage, for_jito_bundle)
quote

Request for a quote to be used in `POST /swap` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**input_mint** | **String** |  | [required] |
**output_mint** | **String** |  | [required] |
**amount** | **i32** | - Raw amount to swap (before decimals) - Input Amount if `SwapMode=ExactIn` - Output Amount if `SwapMode=ExactOut`  | [required] |
**slippage_bps** | Option<**i32**> | - Default: 50 - This is threshold denoted in basis points. - If exact in and output amount exceeds the threshold, then the swap transaction will fail.  |  |[default to 50]
**swap_mode** | Option<**String**> | - ExactOut is for supporting use cases where you need an exact output amount - In the case of `ExactIn`, the slippage is on the output token - In the case of `ExactOut`, the slippage is on the input token - Not all AMMs support `ExactOut`: Currently only Orca Whirlpool, Raydium CLMM, Raydium CPMM - We do not recommend using `ExactOut` for most use cases  |  |[default to ExactIn]
**dexes** | Option<[**Vec<String>**](String.md)> | - Multiple DEXes can be pass in by comma separating them - For example: `dexes=Raydium,Orca+V2,Meteora+DLMM` - If a DEX is indicated, the route will **only use** that DEX - Full list of DEXes here using the `/program-id-to-label` endpoint  |  |
**exclude_dexes** | Option<[**Vec<String>**](String.md)> | - Multiple DEXes can be pass in by comma separating them - For example: `excludeDexes=Raydium,Orca+V2,Meteora+DLMM` - If a DEX is indicated, the route will **not use** that DEX - Full list of DEXes here using the `/program-id-to-label` endpoint  |  |
**restrict_intermediate_tokens** | Option<**bool**> | - Restrict intermediate tokens within a route to a set of more stable tokens - This will help to reduce exposure to potential high slippage routes  |  |[default to true]
**only_direct_routes** | Option<**bool**> | - Direct route limits Jupiter routing to single hop routes only - This may result in worse routes  |  |[default to false]
**as_legacy_transaction** | Option<**bool**> | - Instead of using versioned transaction, this will use the legacy transaction  |  |[default to false]
**platform_fee_bps** | Option<**i32**> | - Take fees in basis points - If `platformFeeBps` is passed in, the `feeAccount` in `/swap` must be passed as well  |  |
**max_accounts** | Option<**i32**> | - Rough estimate of the max accounts to be used for the quote - Useful if composing your own transaction or to be more precise in resource accounting for better routes  |  |[default to 64]
**instruction_version** | Option<**String**> | - The version of instruction to use in the swap program  |  |[default to V1]
**dynamic_slippage** | Option<**bool**> | - No longer applicable, only required to pass in via `/swap` endpoint  |  |[default to false]
**for_jito_bundle** | Option<**bool**> | Set to `true` if the quote will be used in a Jito bundle. This excludes DEXes that are incompatible with Jito bundles (e.g. HumidiFi).  |  |[default to false]

### Return type

[**models::SwapV1QuoteResponse**](SwapV1_QuoteResponse.md)

### Authorization

[SwapV1_ApiKeyAuth](../README.md#SwapV1_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## swap_instructions_post

> models::SwapV1SwapInstructionsResponse swap_instructions_post(swap_v1_swap_request)
swap-instructions

Request for swap instructions that you can use from the quote you get from `/quote` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**swap_v1_swap_request** | [**SwapV1SwapRequest**](SwapV1SwapRequest.md) |  | [required] |

### Return type

[**models::SwapV1SwapInstructionsResponse**](SwapV1_SwapInstructionsResponse.md)

### Authorization

[SwapV1_ApiKeyAuth](../README.md#SwapV1_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## swap_post

> models::SwapV1SwapResponse swap_post(swap_v1_swap_request)
swap

Request for a base64-encoded unsigned swap transaction based on the `/quote` response 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**swap_v1_swap_request** | [**SwapV1SwapRequest**](SwapV1SwapRequest.md) |  | [required] |

### Return type

[**models::SwapV1SwapResponse**](SwapV1_SwapResponse.md)

### Authorization

[SwapV1_ApiKeyAuth](../README.md#SwapV1_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

