# \SwapV2Api

All URIs are relative to *https://api.jup.ag*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_build**](SwapV2Api.md#get_build) | **GET** /swap/v2/build | Build Transaction
[**get_order**](SwapV2Api.md#get_order) | **GET** /swap/v2/order | Get Order
[**post_execute**](SwapV2Api.md#post_execute) | **POST** /swap/v2/execute | Execute Transaction



## get_build

> models::GetBuild200Response get_build(input_mint, output_mint, amount, taker, slippage_bps, mode, dexes, exclude_dexes, platform_fee_bps, fee_account, max_accounts, payer, wrap_and_unwrap_sol, destination_token_account, native_destination_account, blockhash_slots_to_expiry, tip_amount, compute_unit_price_percentile, for_jito_bundle)
Build Transaction

Get a swap quote with raw instructions for building custom transactions. This is the advanced path for integrators who need full transaction control (add custom instructions, CPI, custom payer).  You are responsible for assembling the transaction, signing, and sending via your own RPC. The `/execute` endpoint is not available for `/build` transactions.  Routing is Metis only. No swap fees are charged by Jupiter. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**input_mint** | **String** | The mint address of the input token | [required] |
**output_mint** | **String** | The mint address of the output token | [required] |
**amount** | **String** | The amount to swap in the smallest unit of the input token | [required] |
**taker** | **String** | Public key of the user initiating the swap. | [required] |
**slippage_bps** | Option<[**GetBuildSlippageBpsParameter**](GetBuildSlippageBpsParameter.md)> | Slippage tolerance in basis points. Defaults to 50. |  |[default to 50]
**mode** | Option<**String**> | Quoting mode. \"fast\" trades quote optimality for reduced latency.  |  |
**dexes** | Option<**String**> | - Comma-separated list of DEXes to restrict routing to. - Mutually exclusive with excludeDexes.  |  |
**exclude_dexes** | Option<**String**> | - Comma-separated list of DEXes to exclude. - Mutually exclusive with dexes.  |  |
**platform_fee_bps** | Option<**i32**> | Platform fee in basis points. If positive, feeAccount is required.  |  |
**fee_account** | Option<**String**> | Token account to collect platform fees. Required if platformFeeBps is positive. |  |
**max_accounts** | Option<**i32**> | Maximum number of accounts for the swap route. Defaults to 64. |  |
**payer** | Option<**String**> | Account that pays transaction fees and rent. Defaults to taker when not passed in. |  |
**wrap_and_unwrap_sol** | Option<**bool**> | Whether to wrap/unwrap SOL. Defaults to true. |  |
**destination_token_account** | Option<**String**> | - SPL token account to receive output tokens. - Mutually exclusive with nativeDestinationAccount.  |  |
**native_destination_account** | Option<**String**> | - Native SOL account to receive output. - Mutually exclusive with destinationTokenAccount.  |  |
**blockhash_slots_to_expiry** | Option<**i32**> | Number of slots until the blockhash expires. Defaults to 150. |  |
**tip_amount** | Option<**String**> | SOL tip amount in lamports. Adds a tip instruction for use with /submit. The tip payer is the taker (or payer if specified). |  |
**compute_unit_price_percentile** | Option<[**GetBuildComputeUnitPricePercentileParameter**](GetBuildComputeUnitPricePercentileParameter.md)> | Percentile of compute unit price to use. Named levels: \"medium\" (25th), \"high\" (50th), \"veryHigh\" (75th), or a number 0-10000 in basis points. |  |
**for_jito_bundle** | Option<**bool**> | Set to `true` if the transaction will be submitted as part of a Jito bundle. This excludes DEXes that are incompatible with Jito bundles.  |  |[default to false]

### Return type

[**models::GetBuild200Response**](GetBuild_200_response.md)

### Authorization

[SwapV2_ApiKeyAuth](../README.md#SwapV2_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_order

> models::GetOrder200Response get_order(input_mint, output_mint, amount, taker, receiver, swap_mode, slippage_bps, referral_account, referral_fee, payer, priority_fee_lamports, jito_tip_lamports, broadcast_fee_type, exclude_routers, exclude_dexes)
Get Order

Get a swap quote and assembled transaction. This is the default happy path with all Ultra benefits (predictive execution, RTSE, Jupiter Beam, gasless, multi-router competition).  Without optional parameters, all routers compete for the best price (Metis, JupiterZ RFQ, Dflow, OKX). Setting `payer` to a wallet different from `taker` restricts routing to Metis. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**input_mint** | **String** | The mint address of the input token | [required] |
**output_mint** | **String** | The mint address of the output token | [required] |
**amount** | **String** | The amount to swap in the smallest unit of the input token | [required] |
**taker** | Option<**String**> | - The public key of the wallet that will sign the transaction - If not provided, the response will contain a quote but no transaction - Must be present if you intend to sign and execute the transaction via `/execute`  |  |
**receiver** | Option<**String**> | - The public key of the account that will receive the output tokens - Must differ from `taker` - Expects a wallet address, not a token account - For non-SOL output: tokens are sent to the receiver's associated token account (ATA). If the ATA is not initialised, a create ATA instruction is added to the transaction - For SOL output: native SOL is transferred directly to the receiver - Does not support destination wSOL token accounts  |  |
**swap_mode** | Option<**String**> | - Swap mode. Currently only `ExactIn` is supported.  |  |
**slippage_bps** | Option<**i32**> | - Slippage tolerance in basis points (0-10000) - If not set, Jupiter automatically determines an appropriate slippage  |  |
**referral_account** | Option<**String**> | - Address of your referral account for the Jupiter referral project - Must be used together with `referralFee` - See the [Referral Program](/tool-kits/referral-program) for setup  |  |
**referral_fee** | Option<**f64**> | - Referral fee in basis points (50-255) - Must be used together with `referralAccount`  |  |
**payer** | Option<**String**> | - The public key of an account that will cover gas-related fees (signature fees, priority fees, and rent) on behalf of the taker - Has no effect when set equal to `taker`. Must be a different wallet to activate integrator-sponsored gas - Enabling this restricts routing to Metis (JupiterZ is disabled when an integrator payer is set)  |  |
**priority_fee_lamports** | Option<**f64**> | - Priority fee in lamports - If not set, Jupiter automatically determines an appropriate priority fee - Setting this overrides the automatic optimisation  |  |
**jito_tip_lamports** | Option<**f64**> | - Jito MEV tip in lamports for faster block inclusion  |  |
**broadcast_fee_type** | Option<**String**> | - Fee cap strategy: `maxCap` treats the fee as a maximum, `exactFee` uses the exact amount - Ignored if neither `priorityFeeLamports` nor `jitoTipLamports` are set  |  |
**exclude_routers** | Option<**String**> | - Comma-separated list of routers to exclude - Available routers: `metis`, `jupiterz`, `dflow`, `okx`  |  |
**exclude_dexes** | Option<**String**> | - Comma-separated list of DEXes to exclude from the Metis router - **Important**: This only affects the Metis router, not other routers - For example: `excludeDexes=Raydium,Orca+V2,Meteora+DLMM`  |  |

### Return type

[**models::GetOrder200Response**](GetOrder_200_response.md)

### Authorization

[SwapV2_ApiKeyAuth](../README.md#SwapV2_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_execute

> models::PostExecute200Response post_execute(post_execute_request)
Execute Transaction

Execute a signed transaction from `/order`. Handles transaction landing, confirmation, and retry.  Only available for transactions obtained via `/order`. Transactions built via `/build` must be sent through your own RPC. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_execute_request** | [**PostExecuteRequest**](PostExecuteRequest.md) |  | [required] |

### Return type

[**models::PostExecute200Response**](PostExecute_200_response.md)

### Authorization

[SwapV2_ApiKeyAuth](../README.md#SwapV2_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

