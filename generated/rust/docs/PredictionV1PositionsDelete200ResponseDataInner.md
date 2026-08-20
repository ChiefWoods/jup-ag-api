# PredictionV1PositionsDelete200ResponseDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transaction** | **String** | Base64 encoded transaction | 
**tx_meta** | [**models::PredictionClaimPositionResponseTxMeta**](PredictionClaimPositionResponseTxMeta.md) |  | 
**external_order_id** | Option<**String**> |  | 
**required_signers** | Option<**Vec<String>**> | Public keys that must sign the returned transaction | [optional]
**execution** | Option<[**models::PredictionExecution**](PredictionExecution.md)> |  | [optional]
**execution_model** | Option<**String**> | Execution model for the order. `atomic_swap` for Jupiter Forecast (bisonfi) orders, which execute as a single atomic swap through `POST /execute`. `null` for keeper-filled prediction orders. | [optional]
**settlement** | Option<**String**> | Settlement mode. `auto` for Jupiter Forecast orders, which settle automatically. `null` for keeper-filled prediction orders. | [optional]
**jupiter_swap_request_id** | Option<**String**> | Jupiter swap request id when the order executes as a swap (Jupiter Forecast). `null` otherwise. | [optional]
**order** | [**models::PredictionCreateOrderResponseOrder**](PredictionCreateOrderResponseOrder.md) |  | 
**position** | [**models::PredictionClaimPositionResponsePosition**](PredictionClaimPositionResponsePosition.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


