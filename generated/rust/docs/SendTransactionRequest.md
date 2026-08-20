# SendTransactionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**jsonrpc** | **Jsonrpc** |  (enum: 2.0) | 
**id** | **i32** | Request identifier echoed back in the response | 
**method** | **Method** |  (enum: sendTransaction) | 
**params** | **Vec<serde_json::Value>** | A two-element positional array: `[signedTransaction, config]`.  **`signedTransaction`** — string. The base64-encoded signed transaction.  **`config`** — object. Accepts: - `encoding`: defaults to `base64` (the only supported value) - `skipPreflight`: preflight is not supported; defaults to `true`; an explicit `false` is rejected with `-1015` - `maxRetries`: defaults to `0`; a non-zero value is rejected with `-1015` - `swqosOnly` (optional): `false` (default) uses SWQoS + Jito, `true` uses SWQoS only; see [Tips and fees](/transaction/submit#tips-and-fees) in the guide  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


