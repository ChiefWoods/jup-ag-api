# PredictionHistoryEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | Unique history event identifier | 
**event_type** | **EventType** | Type of history event. Position wins surface as payout_claimed (there is no position_won). (enum: order_created, order_filled, order_failed, order_closed, payout_claimed, position_updated, position_lost) | 
**signature** | **String** | Solana transaction signature | 
**slot** | **String** | Solana slot number (u64 as string) | 
**timestamp** | **i32** | Unix timestamp (seconds) when the event occurred | 
**order_pubkey** | **String** | Associated order account public key | 
**position_pubkey** | **String** | Associated position account public key | 
**market_id** | **String** | External market identifier | 
**owner_pubkey** | **String** | Position or order owner public key | 
**keeper_pubkey** | **String** | Keeper that processed the transaction | 
**external_order_id** | **String** | Client-provided order identifier | 
**order_id** | **String** | External order identifier from the venue | 
**is_buy** | **bool** | True when the event relates to a buy order | 
**is_yes** | **bool** | True when the event relates to the YES side | 
**contracts** | **String** | Number of contracts in the order. Legacy whole-contract value; see contractsMicro/contractsDecimal for the exact quantity. | 
**contracts_micro** | Option<**String**> | Contracts in micro-contract units (u64 as string, 1000000 = 1 contract) | [optional]
**contracts_decimal** | Option<**String**> | Contracts as a decimal string | [optional]
**filled_contracts** | **String** | Number of contracts filled (legacy whole-contract value) | 
**filled_contracts_micro** | Option<**String**> | Filled contracts in micro-contract units | [optional]
**filled_contracts_decimal** | Option<**String**> | Filled contracts as a decimal string | [optional]
**contracts_settled** | **String** | Number of contracts settled in this event (legacy whole-contract value) | 
**contracts_settled_micro** | Option<**String**> | Settled contracts in micro-contract units | [optional]
**contracts_settled_decimal** | Option<**String**> | Settled contracts as a decimal string | [optional]
**max_fill_price_usd** | **String** | Maximum fill price in micro USD (u64 as string) | 
**avg_fill_price_usd** | **String** | Average fill price in micro USD (u64 as string) | 
**max_buy_price_usd** | Option<**String**> | Buyer-specified max fill price in micro USD (u64 as string) | 
**min_sell_price_usd** | Option<**String**> | Seller-specified min fill price in micro USD (u64 as string) | 
**deposit_amount_usd** | **String** | Amount deposited for this order in micro USD (u64 as string) | 
**total_cost_usd** | **String** | Total cost of the order in micro USD (u64 as string) | 
**fee_usd** | Option<**String**> | Fee charged for this event in micro USD (u64 as string) | 
**gross_proceeds_usd** | **String** | Gross proceeds before fees in micro USD (u64 as string) | 
**net_proceeds_usd** | **String** | Net proceeds after fees in micro USD (u64 as string) | 
**transfer_amount_token** | Option<**String**> | Token amount transferred in this event (native token units as string) | 
**realized_pnl** | Option<**String**> | Realized PnL from this event in micro USD (i64 as string) | 
**realized_pnl_before_fees** | Option<**String**> | Realized PnL before fees in micro USD (i64 as string) | 
**payout_amount_usd** | **String** | Payout amount in micro USD (u64 as string), populated for payout_claimed events | 
**event_id** | **String** | External event identifier | 
**market_metadata** | [**models::PredictionHistoryEventMarketMetadata**](PredictionHistoryEventMarketMetadata.md) |  | 
**event_metadata** | [**models::PredictionEventMetadata**](PredictionEventMetadata.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


