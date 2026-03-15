# RotatedSecretCreateAzure

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_id** | Option<**String**> | API ID to rotate (relevant only for rotator-type=api-key) | [optional]
**api_key** | Option<**String**> | API key to rotate (relevant only for rotator-type=api-key) | [optional]
**application_id** | Option<**String**> | Id of the azure app that hold the serect to be rotated (relevant only for rotator-type=api-key & authentication-credentials=use-target-creds) | [optional]
**authentication_credentials** | Option<**String**> | The credentials to connect with use-user-creds/use-target-creds | [optional][default to use-user-creds]
**auto_rotate** | Option<**String**> | Whether to automatically rotate every --rotation-interval days, or disable existing automatic rotation [true/false] | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**explicitly_set_sa** | Option<**String**> | If set, explicitly provide the storage account details [true/false] | [optional][default to false]
**grace_rotation** | Option<**String**> | Enable graceful rotation (keep both versions temporarily). When enabled, a new secret version is created while the previous version is kept for the grace period, so both versions exist for a limited time. [true/false] | [optional]
**grace_rotation_hour** | Option<**i32**> | The Hour of the grace rotation in UTC | [optional]
**grace_rotation_interval** | Option<**String**> | The number of days to wait before deleting the old key (must be bigger than rotation-interval) | [optional]
**grace_rotation_timing** | Option<**String**> | When to create the new version relative to the rotation date [after/before] | [optional]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key** | Option<**String**> | The name of a key that used to encrypt the secret value (if empty, the account default protectionKey key will be used) | [optional]
**max_versions** | Option<**String**> | Set the maximum number of versions, limited by the account settings defaults. | [optional]
**name** | **String** | Rotated secret name | 
**password_length** | Option<**String**> | The length of the password to be generated | [optional]
**resource_group_name** | Option<**String**> | The resource group name (only relevant when explicitly-set-sa=true) | [optional]
**resource_name** | Option<**String**> | The name of the storage account (only relevant when explicitly-set-sa=true) | [optional]
**rotate_after_disconnect** | Option<**String**> | Rotate the value of the secret after SRA session ends [true/false] | [optional][default to false]
**rotation_event_in** | Option<**Vec<String>**> | How many days before the rotation of the item would you like to be notified | [optional]
**rotation_hour** | Option<**i32**> | The Hour of the rotation in UTC | [optional]
**rotation_interval** | Option<**String**> | The number of days to wait between every automatic key rotation (1-365) | [optional]
**rotator_type** | **String** | The rotator type. options: [target/password/api-key/azure-storage-account] | 
**secure_access_disable_concurrent_connections** | Option<**bool**> | Enable this flag to prevent simultaneous use of the same secret | [optional]
**secure_access_enable** | Option<**String**> | Enable/Disable secure remote access [true/false] | [optional]
**secure_access_url** | Option<**String**> | Destination URL to inject secrets | [optional]
**secure_access_web** | Option<**bool**> | Enable Web Secure Remote Access | [optional][default to false]
**secure_access_web_browsing** | Option<**bool**> | Secure browser via Akeyless's Secure Remote Access (SRA) | [optional][default to false]
**secure_access_web_proxy** | Option<**bool**> | Web-Proxy via Akeyless's Secure Remote Access (SRA) | [optional][default to false]
**storage_account_key_name** | Option<**String**> | The name of the storage account key to rotate [key1/key2/kerb1/kerb2] (relevat to azure-storage-account) | [optional]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**target_name** | **String** | The target name to associate | 
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**username** | Option<**String**> | The user principal name to rotate his password (relevant only for rotator-type=password) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


