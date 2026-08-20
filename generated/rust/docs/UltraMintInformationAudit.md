# UltraMintInformationAudit

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_sus** | Option<**bool**> | Flagged as suspicious. This field is only present when the token has been flagged — check for the field's presence, not its value. Not all risky tokens will have this flag.  | [optional]
**mint_authority_disabled** | Option<**bool**> |  | [optional]
**freeze_authority_disabled** | Option<**bool**> |  | [optional]
**top_holders_percentage** | Option<**f64**> | Percentage of supply held by top holders | [optional]
**dev_balance_percentage** | Option<**f64**> | Percentage of supply held by the developer | [optional]
**dev_mints** | Option<**f64**> | Number of developer mint events | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


