# \SendApi

All URIs are relative to *https://api.jup.ag*

Method | HTTP request | Description
------------- | ------------- | -------------
[**send_v1_craft_clawback_post**](SendApi.md#send_v1_craft_clawback_post) | **POST** /send/v1/craft-clawback | craft-clawback
[**send_v1_craft_send_post**](SendApi.md#send_v1_craft_send_post) | **POST** /send/v1/craft-send | craft-send
[**send_v1_invite_history_get**](SendApi.md#send_v1_invite_history_get) | **GET** /send/v1/invite-history | invite-history
[**send_v1_pending_invites_get**](SendApi.md#send_v1_pending_invites_get) | **GET** /send/v1/pending-invites | pending-invites



## send_v1_craft_clawback_post

> models::SendV1CraftClawbackPost200Response send_v1_craft_clawback_post(send_v1_craft_clawback_post_request)
craft-clawback

Request for a base64-encoded unsigned Send transaction 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**send_v1_craft_clawback_post_request** | [**SendV1CraftClawbackPostRequest**](SendV1CraftClawbackPostRequest.md) |  | [required] |

### Return type

[**models::SendV1CraftClawbackPost200Response**](_send_v1_craft_clawback_post_200_response.md)

### Authorization

[Send_ApiKeyAuth](../README.md#Send_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_v1_craft_send_post

> models::SendV1CraftSendPost200Response send_v1_craft_send_post(send_v1_craft_send_post_request)
craft-send

Request for a base64-encoded unsigned Send transaction 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**send_v1_craft_send_post_request** | [**SendV1CraftSendPostRequest**](SendV1CraftSendPostRequest.md) |  | [required] |

### Return type

[**models::SendV1CraftSendPost200Response**](_send_v1_craft_send_post_200_response.md)

### Authorization

[Send_ApiKeyAuth](../README.md#Send_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_v1_invite_history_get

> models::SendInviteDataResponse send_v1_invite_history_get(address, page)
invite-history

Request for the invite history of an address 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | - Pubkey can be sender or recipient  | [required] |
**page** | Option<**i32**> | - Pagination of response  |  |[default to 1]

### Return type

[**models::SendInviteDataResponse**](Send_InviteDataResponse.md)

### Authorization

[Send_ApiKeyAuth](../README.md#Send_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_v1_pending_invites_get

> models::SendInviteDataResponse send_v1_pending_invites_get(address, page)
pending-invites

Request for the pending invites of an address 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | - Pubkey of Sender  | [required] |
**page** | Option<**i32**> | - Pagination of response  |  |[default to 1]

### Return type

[**models::SendInviteDataResponse**](Send_InviteDataResponse.md)

### Authorization

[Send_ApiKeyAuth](../README.md#Send_ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

