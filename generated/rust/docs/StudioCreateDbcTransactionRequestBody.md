# StudioCreateDbcTransactionRequestBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**build_curve_by_market_cap_param** | [**models::StudioCreateDbcTransactionRequestBodyBuildCurveByMarketCapParam**](StudioCreateDBCTransactionRequestBodyBuildCurveByMarketCapParam.md) |  | 
**anti_sniping** | **bool** | - Enable anti-sniping protection - Apply an additional swap fee immediately after launch to discourage bots - Starts at 99% and decreases 1% linearly over a randomized period  | 
**fee** | [**models::StudioCreateDbcTransactionRequestBodyFee**](StudioCreateDBCTransactionRequestBodyFee.md) |  | 
**is_lp_locked** | Option<**bool**> | - Whether LP tokens should be locked - If enabled, 50% of graduated LP unlocks after 1 year - Useful for creators to strategize in the long term  | [optional][default to true]
**token_name** | **String** |  | 
**token_symbol** | **String** |  | 
**token_image_content_type** | **TokenImageContentType** | - Token image content type  (enum: image/jpeg, image/png, image/gif, image/webp) | 
**creator** | **String** | - Creator wallet public key  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


