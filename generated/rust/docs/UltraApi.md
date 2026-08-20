# \UltraApi

All URIs are relative to *https://api.jup.ag*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ultra_v1_balances_address_get**](UltraApi.md#ultra_v1_balances_address_get) | **GET** /ultra/v1/balances/{address} | balances
[**ultra_v1_execute_post**](UltraApi.md#ultra_v1_execute_post) | **POST** /ultra/v1/execute | execute
[**ultra_v1_holdings_address_get**](UltraApi.md#ultra_v1_holdings_address_get) | **GET** /ultra/v1/holdings/{address} | holdings
[**ultra_v1_holdings_address_native_get**](UltraApi.md#ultra_v1_holdings_address_native_get) | **GET** /ultra/v1/holdings/{address}/native | holdings (native)
[**ultra_v1_order_get**](UltraApi.md#ultra_v1_order_get) | **GET** /ultra/v1/order | order
[**ultra_v1_order_routers_get**](UltraApi.md#ultra_v1_order_routers_get) | **GET** /ultra/v1/order/routers | routers
[**ultra_v1_search_get**](UltraApi.md#ultra_v1_search_get) | **GET** /ultra/v1/search | search
[**ultra_v1_shield_get**](UltraApi.md#ultra_v1_shield_get) | **GET** /ultra/v1/shield | shield



## ultra_v1_balances_address_get

> std::collections::HashMap<String, models::UltraV1BalancesAddressGet200ResponseValue> ultra_v1_balances_address_get(address)
balances

Request for token balances of an account 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** |  | [required] |

### Return type

[**std::collections::HashMap<String, models::UltraV1BalancesAddressGet200ResponseValue>**](_ultra_v1_balances__address__get_200_response_value.md)

### Authorization

[Ultra_ApiKeyAuth](../README.md#Ultra_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ultra_v1_execute_post

> models::UltraV1ExecutePost200Response ultra_v1_execute_post(ultra_v1_execute_post_request)
execute

Execute the signed transaction and get the execution status 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ultra_v1_execute_post_request** | Option<[**UltraV1ExecutePostRequest**](UltraV1ExecutePostRequest.md)> |  |  |

### Return type

[**models::UltraV1ExecutePost200Response**](_ultra_v1_execute_post_200_response.md)

### Authorization

[Ultra_ApiKeyAuth](../README.md#Ultra_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ultra_v1_holdings_address_get

> models::UltraHoldingsResponse ultra_v1_holdings_address_get(address)
holdings

Request for token balances of an account including token account information 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The wallet address to get holdings for | [required] |[default to BQ72nSv9f3PRyRKCBnHLVrerrv37CYTHm5h3s9VSGQDV]

### Return type

[**models::UltraHoldingsResponse**](Ultra_HoldingsResponse.md)

### Authorization

[Ultra_ApiKeyAuth](../README.md#Ultra_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ultra_v1_holdings_address_native_get

> models::UltraNativeHoldingsResponse ultra_v1_holdings_address_native_get(address)
holdings (native)

Request for native SOL holdings only, without other tokens 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | The wallet address to get native SOL balance for | [required] |

### Return type

[**models::UltraNativeHoldingsResponse**](Ultra_NativeHoldingsResponse.md)

### Authorization

[Ultra_ApiKeyAuth](../README.md#Ultra_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ultra_v1_order_get

> models::UltraV1OrderGet200Response ultra_v1_order_get(input_mint, output_mint, amount, taker, receiver, payer, close_authority, referral_account, referral_fee, exclude_routers, exclude_dexes)
order

Request for a base64-encoded unsigned swap transaction to be used in `POST /ultra/v1/execute` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**input_mint** | **String** |  | [required] |[default to So11111111111111111111111111111111111111112]
**output_mint** | **String** |  | [required] |[default to EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v]
**amount** | **String** |  | [required] |[default to 10000000]
**taker** | Option<**String**> |  |  |[default to BQ72nSv9f3PRyRKCBnHLVrerrv37CYTHm5h3s9VSGQDV]
**receiver** | Option<**String**> | - The public key of the account that will receive the output tokens - If not provided, the output tokens will be sent to the taker like default - It expects an account (NOT a token account) - If the output is non-SOL tokens, it only transfers to ATAs (not token accounts) - If the destination token account is not initialized, an additional create ATA instruction will be added to the transaction - If the output is SOL, it will transfer native SOL directly to the receiver account - It does not support destination WSOL token account  |  |
**payer** | Option<**String**> | - The public key of an account that will be used to cover 'gas-related' fees on behalf of the taker - Gas related fees such as signature fees, priority fees and rent. Note that enabling this may result in different routing decisions. - Refer to [Integrator Payer](/docs/ultra/add-payer) for more details  |  |
**close_authority** | Option<**String**> | - Optional. If used, requires `payer` - Ignored, if `payer` is not provided - The public key of an account to set as the close authority of ATAs created during the swap transaction. Only applies to non-wSOL ATAs that persist beyond the execution of the transaction. - If `closeAuthority` is not provided, we will default to `taker` - If `closeAuthority` is provided and is different from `taker`, we will add the instruction to set the new `closeAuthority`  |  |
**referral_account** | Option<**String**> | - Refer to [Integrator Fees](/docs/ultra/add-fees-to-ultra) for more details  |  |
**referral_fee** | Option<**f64**> | - Refer to [Integrator Fees](/docs/ultra/add-fees-to-ultra) for more details  |  |
**exclude_routers** | Option<**String**> |  |  |
**exclude_dexes** | Option<**String**> | - [Full list of DEXes here](https://developers.jup.ag/docs/api-reference/swap/program-id-to-label), for example: `excludeDexes=Raydium,Orca+V2,Meteora+DLMM` - **Important**: This only excludes DEXes on the Metis router, does not apply to other routers - For example:   - **Exclude** Raydium: `excludeRouters=<all-except-metis>` and `excludeDexes=Raydium`   - **Only include** Meteora DLMM: `excludeRouters=<all-except-metis>` and `excludeDexes=<all-except-MeteoraDLMM>`  |  |

### Return type

[**models::UltraV1OrderGet200Response**](_ultra_v1_order_get_200_response.md)

### Authorization

[Ultra_ApiKeyAuth](../README.md#Ultra_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ultra_v1_order_routers_get

> Vec<models::UltraV1OrderRoutersGet200ResponseInner> ultra_v1_order_routers_get()
routers

Request for the list of routers available in the routing engine of Ultra, which is [Juno](/docs/routing#juno-liquidity-engine) 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::UltraV1OrderRoutersGet200ResponseInner>**](_ultra_v1_order_routers_get_200_response_inner.md)

### Authorization

[Ultra_ApiKeyAuth](../README.md#Ultra_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ultra_v1_search_get

> Vec<models::UltraMintInformation> ultra_v1_search_get(query)
search

Request a search by token's symbol, name or mint address 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | - Search for a token and its information by its symbol, name or mint address - Comma-separate to search for multiple - Limit to 100 mint addresses in query - Default to 20 mints in response when searching via symbol or name  | [required] |[default to EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v]

### Return type

[**Vec<models::UltraMintInformation>**](Ultra_MintInformation.md)

### Authorization

[Ultra_ApiKeyAuth](../README.md#Ultra_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ultra_v1_shield_get

> models::UltraV1ShieldGet200Response ultra_v1_shield_get(mints)
shield

Request for token information and warnings of mints 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mints** | **String** | - Comma separated list of mints to get information for  | [required] |[default to So11111111111111111111111111111111111111112,EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v]

### Return type

[**models::UltraV1ShieldGet200Response**](_ultra_v1_shield_get_200_response.md)

### Authorization

[Ultra_ApiKeyAuth](../README.md#Ultra_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

