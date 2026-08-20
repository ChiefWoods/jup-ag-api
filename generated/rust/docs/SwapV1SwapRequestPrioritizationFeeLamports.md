# SwapV1SwapRequestPrioritizationFeeLamports

## Enum Variants

| Name | Description |
|---- | -----|
| SwapV1JitoTipLamports | - To specify a level or amount of additional fees to prioritize the transaction - It can be used for EITHER priority fee OR Jito tip (not both at the same time) - If you want to include both, you will need to use &#x60;/swap-instructions&#x60; to add both at the same time - Defaults to &#x60;auto&#x60;, but preferred to use &#x60;priorityLevelWithMaxLamports&#x60; as it may be more accurate when accounting local fee market - Fixed lamports can be passed in as an integer in the &#x60;prioritizationFeeLamports&#x60; parameter  |
| SwapV1JitoTipLamportsWithPayer | - To specify a level or amount of additional fees to prioritize the transaction - It can be used for EITHER priority fee OR Jito tip (not both at the same time) - If you want to include both, you will need to use &#x60;/swap-instructions&#x60; to add both at the same time - Defaults to &#x60;auto&#x60;, but preferred to use &#x60;priorityLevelWithMaxLamports&#x60; as it may be more accurate when accounting local fee market - Fixed lamports can be passed in as an integer in the &#x60;prioritizationFeeLamports&#x60; parameter  |
| SwapV1PriorityLevelWithMaxLamports | - To specify a level or amount of additional fees to prioritize the transaction - It can be used for EITHER priority fee OR Jito tip (not both at the same time) - If you want to include both, you will need to use &#x60;/swap-instructions&#x60; to add both at the same time - Defaults to &#x60;auto&#x60;, but preferred to use &#x60;priorityLevelWithMaxLamports&#x60; as it may be more accurate when accounting local fee market - Fixed lamports can be passed in as an integer in the &#x60;prioritizationFeeLamports&#x60; parameter  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


