# SwapV1SwapInstructionsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**other_instructions** | [**Vec<models::SwapV1Instruction>**](SwapV1Instruction.md) | - If you set jito tips using the `prioritizationFeeLamports` parameter, you will see a custom tip instruction to Jito here.  | 
**compute_budget_instructions** | [**Vec<models::SwapV1Instruction>**](SwapV1Instruction.md) | - To setup the compute budget for the transaction.  | 
**setup_instructions** | [**Vec<models::SwapV1Instruction>**](SwapV1Instruction.md) | - To setup required token accounts for the users.  | 
**swap_instruction** | [**models::SwapV1Instruction**](SwapV1Instruction.md) |  | 
**cleanup_instruction** | Option<[**models::SwapV1Instruction**](SwapV1Instruction.md)> |  | [optional]
**address_lookup_table_addresses** | **Vec<String>** | - The lookup table addresses if you are using versioned transaction.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


