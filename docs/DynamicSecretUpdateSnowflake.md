# DynamicSecretUpdateSnowflake

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account** | Option<**String**> | Account name | [optional]
**account_password** | Option<**String**> | Database Password | [optional]
**account_username** | Option<**String**> | Database Username | [optional]
**auth_mode** | Option<**String**> | The authentication mode for the temporary user [password/key] | [optional][default to password]
**custom_username_template** | Option<**String**> | Customize how temporary usernames are generated using go template | [optional]
**db_name** | Option<**String**> | Database name | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key_algo** | Option<**String**> |  | [optional]
**name** | **String** | Dynamic secret name | 
**new_name** | Option<**String**> | Dynamic secret name | [optional]
**password_length** | Option<**String**> | The length of the password to be generated | [optional]
**private_key** | Option<**String**> | RSA Private key (base64 encoded) | [optional]
**private_key_passphrase** | Option<**String**> | The Private key passphrase | [optional]
**role** | Option<**String**> | User role | [optional]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**target_name** | Option<**String**> | Target name | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**user_ttl** | Option<**String**> | User TTL | [optional][default to 24h]
**warehouse** | Option<**String**> | Warehouse name | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


