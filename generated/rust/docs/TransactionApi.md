# \TransactionApi

All URIs are relative to *https://api.jup.ag*

Method | HTTP request | Description
------------- | ------------- | -------------
[**send_transaction**](TransactionApi.md#send_transaction) | **POST** / | Send a transaction



## send_transaction

> models::SendTransaction200Response send_transaction(send_transaction_request)
Send a transaction

`tx.jup.ag` is a Solana RPC-compatible endpoint (compatible where possible) that forwards signed transactions to the Solana cluster through Jupiter's landing infrastructure. It implements the standard Solana [`sendTransaction`](https://solana.com/docs/rpc/http/sendtransaction) method, so any Solana client can point at it. A success response means the transaction was accepted and forwarded, not that it landed; confirm landing on your own RPC.  It is send-only: `getLatestBlockhash`, `simulateTransaction`, and confirmation queries are not served. Keep your own RPC for those.  **Authentication:** requires your Jupiter API key in the `x-api-key` header.  **Validation:** - Transaction must be a valid signed Solana transaction with all required signatures - Transaction must contain a SOL transfer of >= 1,000,000 lamports (0.001 SOL) to one of the 16 tip receiver accounts - Transaction must not exceed the Solana transaction size limit  **Tip receiver accounts (16 Jupiter V6 program authorities):**  `GGztQqQ6pCPaJQnNpXBgELr5cs3WwDakRbh1iEMzjgSJ`, `2MFoS3MPtvyQ4Wh4M9pdfPjz6UhVoNbFbGJAskCPCj3h`, `BQ72nSv9f3PRyRKCBnHLVrerrv37CYTHm5h3s9VSGQDV`, `6U91aKa8pmMxkJwBCfPTmUEfZi6dHe7DcFq2ALvB2tbB`, `4xDsmeTWPNjgSVSS1VTfzFq3iHZhp77ffPkAmkZkdu71`, `CapuXNQoDviLvU1PxFiizLgPNQCxrsag1uMeyk6zLVps`, `9nnLbotNTcUhvbrsA6Mdkx45Sm82G35zo28AqUvjExn8`, `6LXutJvKUw8Q5ue2gCgKHQdAN4suWW8awzFVC6XCguFx`, `HFqp6ErWHY6Uzhj8rFyjYuDya2mXUpYEk8VW75K9PSiY`, `DSN3j1ykL3obAVNv7ZX49VsFCPe4LqzxHnmtLiPwY6xg`, `69yhtoJR4JYPPABZcSNkzuqbaFbwHsCkja1sP1Q2aVT5`, `HU23r7UoZbqTUuh3vA7emAGztFtqwTeVips789vqxxBw`, `3LoAYHuSd7Gh8d7RTFnhvYtiTiefdZ5ByamU42vkzd76`, `3CgvbiM3op4vjrrjH2zcrQUwsqh5veNVRjFCB9N6sRoD`, `GP8StUXNYSZjPikyRsvkTbvRV1GBxMErb59cpeCJnDf1`, `7iWnBRRhBCiNXXPhqiGzvvBkKrvFSWqqmxRyu9VyYBxE`  Randomise which account you send to across transactions to reduce write-lock contention. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**send_transaction_request** | Option<[**SendTransactionRequest**](SendTransactionRequest.md)> |  |  |

### Return type

[**models::SendTransaction200Response**](sendTransaction_200_response.md)

### Authorization

[Transaction_ApiKeyAuth](../README.md#Transaction_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

