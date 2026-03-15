# GatewayCreateProducerCustom

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**admin_rotation_interval_days** | Option<**i64**> | Define rotation interval in days | [optional]
**create_sync_url** | **String** | URL of an endpoint that implements /sync/create method, for example https://webhook.example.com/sync/create | 
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**enable_admin_rotation** | Option<**bool**> | Should admin credentials be rotated | [optional][default to false]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Dynamic secret name | 
**payload** | Option<**String**> | Secret payload to be sent with each create/revoke webhook request | [optional]
**producer_encryption_key_name** | Option<**String**> | Dynamic producer encryption key | [optional]
**revoke_sync_url** | **String** | URL of an endpoint that implements /sync/revoke method, for example https://webhook.example.com/sync/revoke | 
**rotate_sync_url** | Option<**String**> | URL of an endpoint that implements /sync/rotate method, for example https://webhook.example.com/sync/rotate | [optional]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**timeout_sec** | Option<**i64**> | Maximum allowed time in seconds for the webhook to return the results | [optional][default to 60]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**user_ttl** | Option<**String**> | User TTL | [optional][default to 60m]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


