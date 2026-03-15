# RotatedSecretCreateSnowflake

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authentication_credentials** | Option<**String**> | The credentials to connect with use-user-creds/use-target-creds | [optional][default to use-user-creds]
**auto_rotate** | Option<**String**> | Whether to automatically rotate every --rotation-interval days, or disable existing automatic rotation [true/false] | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key** | Option<**String**> | The name of a key that used to encrypt the secret value (if empty, the account default protectionKey key will be used) | [optional]
**max_versions** | Option<**String**> | Set the maximum number of versions, limited by the account settings defaults. | [optional]
**name** | **String** | Rotated secret name | 
**password_length** | Option<**String**> | The length of the password to be generated | [optional]
**private_key** | Option<**String**> | RSA Private key (base64 encoded) to rotate (relevant only for rotator-type=key) | [optional]
**private_key_file_name** | Option<**String**> | The path to the file containing the private key (relevant only for rotator-type=key) | [optional]
**rotated_password** | Option<**String**> | rotated-username password (relevant only for rotator-type=password) | [optional]
**rotated_username** | Option<**String**> | username to be rotated, if selected use-self-creds at rotator-creds-type, this username will try to rotate it's own password, if use-target-creds is selected, target credentials will be use to rotate the rotated-password (relevant only for rotator-type=password or rotator-type=key) | [optional]
**rotation_event_in** | Option<**Vec<String>**> | How many days before the rotation of the item would you like to be notified | [optional]
**rotation_hour** | Option<**i32**> | The Hour of the rotation in UTC | [optional]
**rotation_interval** | Option<**String**> | The number of days to wait between every automatic key rotation (1-365) | [optional]
**rotator_type** | **String** | The rotator type. options: [target/password/key] | 
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**target_name** | **String** | The target name to associate | 
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


