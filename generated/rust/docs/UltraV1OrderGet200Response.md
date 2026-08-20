# UltraV1OrderGet200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mode** | **String** |  | 
**input_mint** | **String** |  | 
**output_mint** | **String** |  | 
**in_amount** | **String** |  | 
**out_amount** | **String** |  | 
**in_usd_value** | Option<**f64**> |  | [optional]
**out_usd_value** | Option<**f64**> |  | [optional]
**price_impact** | Option<**f64**> |  | [optional]
**swap_usd_value** | Option<**f64**> |  | [optional]
**other_amount_threshold** | **String** |  | 
**swap_mode** | **String** |  | 
**slippage_bps** | **f64** |  | 
**price_impact_pct** | **String** | - Please use `priceImpact` field instead, this is still available only for backwards compatibility  | 
**route_plan** | [**Vec<models::UltraV1OrderGet200ResponseRoutePlanInner>**](UltraV1OrderGet200ResponseRoutePlanInner.md) |  | 
**referral_account** | Option<**String**> |  | [optional]
**fee_mint** | Option<**String**> |  | [optional]
**fee_bps** | **f64** | - The fee includes either Ultra default fee or your integrator fee, depending on if referral and/or payer params are passed in - If `referralAccount`, `referralFee` (and `payer` if passed in) are passed in, the fee will be the integrator fee only - If no additional referral params are passed in, the fee will be the Ultra default fee only and can include additional fees for gasless support mechanism  | 
**platform_fee** | [**models::UltraV1OrderGet200ResponsePlatformFee**](UltraV1OrderGet200ResponsePlatformFee.md) |  | 
**signature_fee_lamports** | **f64** | - The number of lamports the `taker`, `maker` (JupiterZ's MM) or `payer` has to pay for the base network fee, if a valid transaction is returned.  | 
**signature_fee_payer** | Option<**String**> | - The public key of the account that will cover the signature fee, it can be either `taker`, `maker` (JupiterZ's MM) or `payer` if passed in  | 
**prioritization_fee_lamports** | **f64** | - The number of lamports the `taker`, `maker` (JupiterZ's MM) or `payer` has to pay for higher priority landing, if a valid transaction is returned - Includes priority fees and tips for services such as Jito, etc, if any  | 
**prioritization_fee_payer** | Option<**String**> | - The public key of the account that will cover the prioritization fee, it can be either `taker`, `maker` (JupiterZ's MM) or `payer` if passed in  | 
**rent_fee_lamports** | **f64** | - The number of lamports the `taker` or `payer` has to pay for account rent, if a valid transaction is returned - Note that this value is just an estimate - For JupiterZ quotes, Jupiter's gas wallet pays the rent to create the taker's output token account if not yet initialised  | 
**rent_fee_payer** | Option<**String**> | - The public key of the account that will cover the rent fee, it can be either `taker`, `payer` if passed in, or Jupiter's gas wallet for JupiterZ quotes when the taker's output token account is not yet initialised  | 
**swap_type** | **String** | - Deprecated, in favour of router  | 
**router** | **Router** |  (enum: metis, jupiterz, dflow, okx) | 
**transaction** | Option<**String**> | - Unsigned base-64 encoded transaction to be signed and used in `/execute` - If `taker` is null, this field will be null. - If the `transaction` field is empty, it is returned with `errorCode` and `errorMessage` such as Insufficient Funds  | 
**gasless** | **bool** |  | 
**request_id** | **String** | - Required to make a request to `/execute`  | 
**total_time** | **f64** |  | 
**taker** | Option<**String**> |  | 
**quote_id** | Option<**String**> |  | [optional]
**maker** | Option<**String**> |  | [optional]
**expire_at** | Option<**String**> |  | [optional]
**error_code** | Option<**ErrorCode**> | - This field will be present if `taker` is defined and `transaction` is an empty string - It is unique for each error scenarios  (enum: 1, 2, 3) | [optional]
**error_message** | Option<**ErrorMessage**> | - This field will be present if `taker` is defined and `transaction` is an empty string - This field can still return despite having a valid order/quote - This is meant for display purposes only and it is discouraged to match these error messages as they could be parameterized  (enum: Insufficient funds, Top up `${solAmount}` SOL for gas, Minimum `${swapAmount}` for gasless) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


