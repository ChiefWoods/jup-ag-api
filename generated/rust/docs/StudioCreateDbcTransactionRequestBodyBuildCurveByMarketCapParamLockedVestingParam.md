# StudioCreateDbcTransactionRequestBodyBuildCurveByMarketCapParamLockedVestingParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_locked_vesting_amount** | **f64** | - Total locked vesting amount of tokens (0% to 80% of total supply)  | 
**cliff_unlock_amount** | **f64** | - Amount of tokens unlocked at cliff  | 
**number_of_vesting_period** | **NumberOfVestingPeriod** | - Number of vesting periods in days (183/365) or months (6/12)  (enum: 0, 6, 12, 183, 365) | 
**total_vesting_duration** | **TotalVestingDuration** | - Total vesting duration in seconds  (enum: 0, 15552000, 15811200, 31536000) | 
**cliff_duration_from_migration_time** | **CliffDurationFromMigrationTime** | - Cliff duration from migration time in seconds  (enum: 0, 15552000, 15811200, 31536000) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


