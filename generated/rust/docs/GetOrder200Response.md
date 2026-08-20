# GetOrder200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mode** | Option<**String**> | \"ultra\" or \"manual\" based on parameters used | [optional]
**input_mint** | Option<**String**> |  | [optional]
**output_mint** | Option<**String**> |  | [optional]
**in_amount** | Option<**String**> |  | [optional]
**out_amount** | Option<**String**> |  | [optional]
**in_usd_value** | Option<**f64**> |  | [optional]
**out_usd_value** | Option<**f64**> |  | [optional]
**price_impact** | Option<**f64**> | Price impact in percentage points (e.g. -0.1 = -0.1%). Divide by 100 to convert to a decimal fraction. | [optional]
**swap_usd_value** | Option<**f64**> |  | [optional]
**other_amount_threshold** | Option<**String**> | Minimum output amount after slippage | [optional]
**swap_mode** | Option<**String**> |  | [optional]
**slippage_bps** | Option<**f64**> |  | [optional]
**price_impact_pct** | Option<**String**> | Deprecated: decimal price impact ratio. Use priceImpact divided by 100 instead. | [optional]
**route_plan** | Option<[**Vec<models::SwapV2RoutePlanStep>**](SwapV2RoutePlanStep.md)> |  | [optional]
**referral_account** | Option<**String**> |  | [optional]
**fee_mint** | Option<**String**> |  | [optional]
**fee_bps** | Option<**f64**> | Total fee rate charged for the swap in basis points. This includes the Jupiter platform fee plus any additional charges, such as gasless support cost recoup. | [optional]
**platform_fee** | Option<[**models::SwapV2PlatformFee**](SwapV2PlatformFee.md)> |  | [optional]
**signature_fee_lamports** | Option<**f64**> |  | [optional]
**signature_fee_payer** | Option<**String**> |  | [optional]
**prioritization_fee_lamports** | Option<**f64**> | Includes priority fees and tips (Jito, Nozomi) | [optional]
**prioritization_fee_payer** | Option<**String**> |  | [optional]
**rent_fee_lamports** | Option<**f64**> | Estimated rent fee | [optional]
**rent_fee_payer** | Option<**String**> |  | [optional]
**swap_type** | Option<**SwapType**> | Deprecated: use router instead (enum: aggregator, rfq, aggregator+rfq, dflow, okx) | [optional]
**router** | Option<**Router**> | Which router won the quote (enum: metis, jupiterz, dflow, okx) | [optional]
**transaction** | Option<**String**> | - Base64-encoded transaction. Null if taker is not provided. - Empty string if taker is provided but transaction could not be built (check errorCode).  | [optional]
**last_valid_block_height** | Option<**String**> |  | [optional]
**gasless** | Option<**bool**> | True when signature and priority fees are paid by a wallet other than the taker. Set by three independent paths: - Automatic Jupiter sponsorship: `signatureFeePayer = gasTzr94Pmp4Gf8vknQnqxeYxdgwFjbgdJa4msYRpnB` - JupiterZ market maker: `signatureFeePayer` is the MM address (varies per quote); Jupiter's gas wallet can pay the rent to create the taker's output token account when referral fees are not set - Integrator payer: `signatureFeePayer` is your `payer` parameter For a deterministic opt-out, check `signatureFeePayer == taker` on every response.  | [optional]
**request_id** | Option<**String**> | Unique request ID. Pass this to /execute. | [optional]
**total_time** | Option<**f64**> | Response time in milliseconds | [optional]
**taker** | Option<**String**> |  | [optional]
**quote_id** | Option<**String**> | Quote ID for RFQ swaps | [optional]
**maker** | Option<**String**> | Market maker address for RFQ swaps | [optional]
**expire_at** | Option<**String**> | Quote expiration timestamp for RFQ swaps | [optional]
**error_code** | Option<**f64**> | Present when `taker` is defined and `transaction` is the empty string. The router quoted a price but could not build a transaction. Match on `router` + `errorCode` to identify the error.  Aggregator routers (router is metis, dflow, okx): - 1: Insufficient funds - 2: Insufficient SOL for gas - 3: Swap below minimum for gasless  JupiterZ router (router is jupiterz): - 1: Insufficient balance to fund the swap - 2: Missing associated token account - 3: Quote could not be built into a transaction  | [optional]
**error_message** | Option<**String**> | Human-readable error description. Present when `taker` is defined and `transaction` is the empty string. Match on `router` + `errorCode` instead of this string, as the message text may be parameterised.  | [optional]
**error** | Option<**String**> | Duplicate of errorMessage for backwards compatibility | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


