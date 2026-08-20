# SwapV1PriorityLevelWithMaxLamportsPriorityLevelWithMaxLamports

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**priority_level** | **PriorityLevel** |  (enum: medium, high, veryHigh) | 
**max_lamports** | **i32** | - Maximum lamports to cap the priority fee estimation, to prevent overpaying  | 
**global** | Option<**bool**> | - A boolean to choose between using a global or local fee market to estimate. If `global` is set to `false`, the estimation focuses on fees relevant to the **writable accounts** involved in the instruction.  | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


