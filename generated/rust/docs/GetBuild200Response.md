# GetBuild200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**input_mint** | Option<**String**> |  | [optional]
**output_mint** | Option<**String**> |  | [optional]
**in_amount** | Option<**String**> |  | [optional]
**out_amount** | Option<**String**> |  | [optional]
**other_amount_threshold** | Option<**String**> | Minimum output amount after slippage | [optional]
**swap_mode** | Option<**String**> |  | [optional]
**slippage_bps** | Option<**f64**> |  | [optional]
**route_plan** | Option<[**Vec<models::SwapV2RoutePlanStep>**](SwapV2RoutePlanStep.md)> |  | [optional]
**compute_budget_instructions** | Option<[**Vec<models::SwapV2Instruction>**](SwapV2Instruction.md)> | Compute unit price instruction (does not include compute unit limit) | [optional]
**setup_instructions** | Option<[**Vec<models::SwapV2Instruction>**](SwapV2Instruction.md)> | Pre-swap setup instructions (e.g. create ATAs) | [optional]
**swap_instruction** | Option<[**models::SwapV2Instruction**](SwapV2Instruction.md)> |  | [optional]
**cleanup_instruction** | Option<[**models::SwapV2Instruction**](SwapV2Instruction.md)> |  | [optional]
**other_instructions** | Option<[**Vec<models::SwapV2Instruction>**](SwapV2Instruction.md)> |  | [optional]
**tip_instruction** | Option<[**models::SwapV2Instruction**](SwapV2Instruction.md)> |  | [optional]
**addresses_by_lookup_table_address** | Option<[**std::collections::HashMap<String, Vec<String>>**](Vec.md)> | Address lookup table mappings for v0 transactions | [optional]
**blockhash_with_metadata** | Option<[**models::GetBuild200ResponseBlockhashWithMetadata**](GetBuild200ResponseBlockhashWithMetadata.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


