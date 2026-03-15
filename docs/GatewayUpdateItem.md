# GatewayUpdateItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**add_tag** | Option<**Vec<String>**> | List of the new tags that will be attached to this item | [optional]
**api_id** | Option<**String**> | API ID to rotate (relevant only for rotator-type=api-key) | [optional]
**api_key** | Option<**String**> | API key to rotate (relevant only for rotator-type=api-key) | [optional]
**app_id** | Option<**String**> | ApplicationId (used in azure) | [optional]
**auto_rotate** | Option<**String**> | Whether to automatically rotate every --rotation-interval days, or disable existing automatic rotation [true/false] | [optional]
**custom_payload** | Option<**String**> | Secret payload to be sent with rotation request (relevant only for rotator-type=custom) | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional][default to default_metadata]
**gcp_key** | Option<**String**> | Base64-encoded service account private key text | [optional]
**gcp_service_account_email** | Option<**String**> | The email of the gcp service account to rotate | [optional]
**gcp_service_account_key_id** | Option<**String**> | The key id of the gcp service account to rotate | [optional]
**grace_rotation** | Option<**String**> | Create a new access key without deleting the old key from AWS for backup (relevant only for AWS) [true/false] | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**keep_prev_version** | Option<**String**> | Whether to keep previous version [true/false]. (relevant only for --type=rotated-secret). If not set, use default according to account settings | [optional]
**key** | Option<**String**> | The name of a key that used to encrypt the secret value (if empty, the account default protectionKey key will be used) | [optional]
**name** | **String** | Item name | 
**new_metadata** | Option<**String**> | Deprecated - use description | [optional][default to default_metadata]
**new_name** | Option<**String**> | New item name | [optional]
**new_version** | Option<**bool**> | Deprecated | [optional]
**password_length** | Option<**String**> | The length of the password to be generated | [optional]
**rm_tag** | Option<**Vec<String>**> | List of the existent tags that will be removed from this item | [optional]
**rotated_password** | Option<**String**> | rotated-username password (relevant only for rotator-type=password) | [optional]
**rotated_username** | Option<**String**> | username to be rotated, if selected \\\"use-self-creds\\\" at rotator-creds-type, this username will try to rotate it's own password, if \\\"use-target-creds\\\" is selected, target credentials will be use to rotate the rotated-password (relevant only for rotator-type=password) | [optional]
**rotation_event_in** | Option<**Vec<String>**> | How many days before the rotation of the item would you like to be notified | [optional]
**rotation_hour** | Option<**i32**> | The Rotation Hour | [optional][default to 0]
**rotation_interval** | Option<**String**> | The number of days to wait between every automatic key rotation (1-365) | [optional]
**rotator_creds_type** | Option<**String**> | The rotation credentials type | [optional][default to use-self-creds]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**r#type** | **String** | Item type | 
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


