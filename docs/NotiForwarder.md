# NotiForwarder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auth_type** | Option<**String**> |  | [optional]
**client_id** | Option<**String**> | Auth - JWT | [optional]
**client_permissions** | Option<**Vec<String>**> |  | [optional]
**comment** | Option<**String**> |  | [optional]
**creation_date** | Option<**String**> |  | [optional]
**endpoint** | Option<**String**> |  | [optional]
**event_types** | Option<**Vec<String>**> |  | [optional]
**gateway_cluster_id** | Option<**i64**> |  | [optional]
**include_error** | Option<**bool**> |  | [optional]
**is_enabled** | Option<**bool**> |  | [optional]
**last_version** | Option<**i32**> |  | [optional]
**modification_date** | Option<**String**> |  | [optional]
**noti_forwarder_id** | Option<**i64**> |  | [optional]
**noti_forwarder_name** | Option<**String**> |  | [optional]
**noti_forwarder_type** | Option<**String**> |  | [optional]
**noti_forwarder_versions** | Option<[**Vec<models::ItemVersion>**](ItemVersion.md)> |  | [optional]
**override_url** | Option<**String**> |  | [optional]
**paths** | Option<**Vec<String>**> |  | [optional]
**protection_key** | Option<**String**> |  | [optional]
**runner_type** | Option<**String**> |  | [optional]
**slack_noti_forwarder_public_details** | Option<[**serde_json::Value**](.md)> |  | [optional]
**teams_noti_forwarder_public_details** | Option<[**serde_json::Value**](.md)> |  | [optional]
**timespan_in_seconds** | Option<**i64**> |  | [optional]
**to_emails** | Option<[**Vec<models::EmailEntry>**](EmailEntry.md)> |  | [optional]
**user_email** | Option<**String**> |  | [optional]
**username** | Option<**String**> | Auth - User Password | [optional]
**webhook_noti_forwarder_public_details** | Option<[**models::WebHookNotiForwarderPublicDetails**](WebHookNotiForwarderPublicDetails.md)> |  | [optional]
**with_customer_fragment** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


