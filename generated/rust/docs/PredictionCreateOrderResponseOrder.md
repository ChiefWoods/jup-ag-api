# PredictionCreateOrderResponseOrder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_pubkey** | Option<**String**> | Order account public key | 
**order_ata_pubkey** | Option<**String**> | Order escrow token account | 
**user_pubkey** | **String** | Order owner public key | 
**market_id** | **String** | Associated market identifier | 
**market_id_hash** | **String** | Hashed market identifier used for on-chain PDAs | 
**position_pubkey** | **String** | Position PDA used for the order | 
**is_buy** | **bool** | True when the order increases exposure | 
**is_yes** | **bool** | True when the order is on the YES side | 
**contracts** | **String** | Legacy whole-contract quantity. Use contractsMicro or contractsDecimal for the exact fractional quantity. | 
**contracts_micro** | Option<**String**> | Contracts in micro-contract units (u64 as string, 1000000 = 1 contract) | [optional]
**contracts_decimal** | Option<**String**> | Contracts as a decimal string | [optional]
**new_contracts** | **String** | Position contracts after executing the order (legacy whole-contract quantity) | 
**new_contracts_micro** | Option<**String**> | Position contracts after the order, in micro-contract units | [optional]
**new_contracts_decimal** | Option<**String**> | Position contracts after the order, as a decimal string | [optional]
**max_buy_price_usd** | Option<**String**> | Max fill price for buys (micro USD) | 
**min_sell_price_usd** | Option<**String**> | Min fill price for sells (micro USD) | 
**external_order_id** | Option<**String**> | Client provided order identifier | 
**order_cost_usd** | **String** | Notional for this order in micro USD | 
**new_avg_price_usd** | **String** | Average position entry price after the order | 
**new_size_usd** | **String** | Total position cost basis after the order | 
**new_payout_usd** | **String** | Position max payout after the order | 
**estimated_protocol_fee_usd** | **String** | Estimated protocol fee (micro USD) | 
**estimated_venue_fee_usd** | **String** | Estimated venue fee (micro USD) | 
**estimated_total_fee_usd** | **String** | Estimated total fees applied (micro USD) | 
**payout_usd** | Option<**String**> | Position max payout for this order (micro USD) | [optional]
**slippage_bps** | Option<**i32**> | Estimated slippage for this order, in basis points (0 = none, 250 = 2.5%). For a Polymarket buy this is the estimated price impact from sweeping orderbook levels; for a sell it is the max allowed slippage from best bid to the protected sell floor. Treat it as a quote signal, not a guaranteed loss. | [optional]
**max_slippage_bps** | Option<**i32**> | Maximum slippage allowed for this order, in basis points | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


