# DynamicSecretUpdateGcp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_type** | Option<**String**> |  | [optional]
**custom_username_template** | Option<**String**> | Customize how temporary usernames are generated using go template | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**fixed_user_claim_keyname** | Option<**String**> | For externally provided users, denotes the key-name of IdP claim to extract the username from (Relevant only when --access-type=external) | [optional][default to ext_email]
**gcp_cred_type** | Option<**String**> |  | [optional]
**gcp_key** | Option<**String**> | Base64-encoded service account private key text | [optional]
**gcp_key_algo** | Option<**String**> | Service account key algorithm, e.g. KEY_ALG_RSA_1024 (Relevant only when --access-type=sa and --gcp-cred-type=key) | [optional]
**gcp_project_id** | Option<**String**> | GCP Project ID override for dynamic secret operations | [optional]
**gcp_sa_email** | Option<**String**> | The email of the fixed service account to generate keys or tokens for (Relevant only when --access-type=sa and --service-account-type=fixed) | [optional]
**gcp_token_scopes** | Option<**String**> | Access token scopes list, e.g. scope1,scope2 (Relevant only when --access-type=sa; required when --gcp-cred-type=token) | [optional]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Dynamic secret name | 
**new_name** | Option<**String**> | Dynamic secret name | [optional]
**producer_encryption_key_name** | Option<**String**> | Dynamic producer encryption key | [optional]
**role_binding** | Option<**String**> | Role binding definitions in JSON format (Relevant only when --access-type=sa and --service-account-type=dynamic) | [optional]
**role_names** | Option<**String**> | Comma-separated list of GCP roles to assign to the user (Relevant only when --access-type=external) | [optional]
**secure_access_delay** | Option<**i64**> | The delay duration, in seconds, to wait after generating just-in-time credentials. Accepted range: 0-120 seconds | [optional]
**secure_access_enable** | Option<**String**> | Enable/Disable secure remote access [true/false] | [optional]
**secure_access_url** | Option<**String**> | Destination URL to inject secrets | [optional]
**secure_access_web_browsing** | Option<**bool**> | Secure browser via Akeyless's Secure Remote Access (SRA) | [optional][default to false]
**secure_access_web_proxy** | Option<**bool**> | Web-Proxy via Akeyless's Secure Remote Access (SRA) | [optional][default to false]
**service_account_type** | Option<**String**> | The type of the GCP service account. Options [fixed, dynamic] (Relevant only when --access-type=sa) | [optional][default to fixed]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**target_name** | Option<**String**> | Target name | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**user_ttl** | Option<**String**> | User TTL | [optional][default to 60m]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


