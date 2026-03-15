# DynamicSecretCreateGoogleWorkspace

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_mode** | **String** |  | 
**admin_email** | **String** | Admin user email | 
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**fixed_user_claim_keyname** | Option<**String**> | For externally provided users, denotes the key-name of IdP claim to extract the username from | [optional][default to ext_email]
**gcp_key** | Option<**String**> | Base64-encoded service account private key text | [optional]
**group_email** | Option<**String**> | A group email, relevant only for group access-mode | [optional]
**group_role** | Option<**String**> |  | [optional]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Dynamic secret name | 
**producer_encryption_key_name** | Option<**String**> | Dynamic producer encryption key | [optional]
**role_name** | Option<**String**> | Name of the admin role to assign to the user, relevant only for role access-mode | [optional]
**role_scope** | Option<**String**> |  | [optional]
**secure_access_delay** | Option<**i64**> | The delay duration, in seconds, to wait after generating just-in-time credentials. Accepted range: 0-120 seconds | [optional]
**secure_access_enable** | Option<**String**> | Enable/Disable secure remote access [true/false] | [optional]
**secure_access_url** | Option<**String**> | Destination URL to inject secrets | [optional]
**secure_access_web** | Option<**bool**> | Enable Web Secure Remote Access | [optional][default to true]
**secure_access_web_browsing** | Option<**bool**> | Secure browser via Akeyless's Secure Remote Access (SRA) | [optional][default to false]
**secure_access_web_proxy** | Option<**bool**> | Web-Proxy via Akeyless's Secure Remote Access (SRA) | [optional][default to false]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**target_name** | Option<**String**> | Name of existing target to use in dynamic secret creation | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**user_ttl** | Option<**String**> | User TTL | [optional][default to 60m]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


