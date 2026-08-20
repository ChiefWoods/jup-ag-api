# PostExecute200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<**Status**> |  (enum: Success, Failed) | [optional]
**signature** | Option<**String**> | Transaction signature | [optional]
**slot** | Option<**String**> | Confirmed slot | [optional]
**error** | Option<**String**> | Error message (present if Failed) | [optional]
**code** | Option<**f64**> | - Error code. 0 = success. - Ultra: -1 (missing cached order), -2 (invalid signed tx), -3 (invalid message bytes). - Aggregator: -1000 (failed to land), -1001 (unknown), -1002 (invalid tx), -1003 (not fully signed), -1004 (invalid block height). - RFQ: -2000 (failed to land), -2001 (unknown), -2002 (invalid payload), -2003 (quote expired), -2004 (swap rejected).  | [optional]
**total_input_amount** | Option<**String**> | Total input token amount deducted from the user's wallet, including the fee if feeMint is the input mint | [optional]
**total_output_amount** | Option<**String**> | Final output token amount reflected in the user's wallet, after any fee collected in the output mint | [optional]
**input_amount_result** | Option<**String**> | Input amount that went into the swap route, after any fee collected in the input mint | [optional]
**output_amount_result** | Option<**String**> | Output amount produced by the swap route, before any fee collected in the output mint | [optional]
**swap_events** | Option<[**Vec<models::SwapV2SwapEvent>**](SwapV2SwapEvent.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


