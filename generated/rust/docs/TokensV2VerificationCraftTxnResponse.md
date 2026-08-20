# TokensV2VerificationCraftTxnResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transaction** | **String** | Base64-encoded unsigned transaction | 
**request_id** | **String** | Unique request ID. Pass this to the execute endpoint. | 
**receiver_address** | **String** | Wallet address receiving the JUP payment | 
**mint** | **String** | Mint address of the payment token (JUP) | 
**amount** | **String** | JUP amount collected in smallest unit. JUP path transfers exactly 1000 JUP (1000000000); swap paths yield at least that. | 
**token_decimals** | **f64** | Decimals of the payment token | 
**token_usd_rate** | Option<**f64**> | USD rate of the payment token at time of crafting | [optional]
**fee_lamports** | **f64** | Transaction fee in lamports | 
**fee_usd_amount** | Option<**f64**> | Transaction fee in USD | [optional]
**fee_mint** | **String** | Mint address of the fee token | 
**fee_token_decimals** | **f64** | Decimals of the fee token | 
**fee_amount** | **f64** | Fee amount in smallest unit | 
**total_time** | **f64** | Time taken to craft the transaction in milliseconds | 
**expire_at** | Option<**String**> | Expiry timestamp for the crafted transaction | [optional]
**error** | Option<**String**> | Error message if crafting failed | [optional]
**code** | **f64** | Status code (0 for success) | 
**gasless** | **bool** | Whether the transaction is gasless | 
**input_mint** | Option<**String**> | Input token mint address (non-JUP paths) | [optional]
**input_decimals** | Option<**f64**> | Input token decimals (non-JUP paths) | [optional]
**quoted_input_amount** | Option<**String**> | Atomic input amount for the swap, sized to yield at least 1000 JUP (non-JUP paths) | [optional]
**max_input_amount** | Option<**String**> | Maximum atomic input amount for the swap; equal to quotedInputAmount on the current swap flow (non-JUP paths) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


