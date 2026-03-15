# DynamicSecretCreateAzure

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**app_obj_id** | Option<**String**> | Azure App Object Id | [optional]
**azure_administrative_unit** | Option<**String**> | Azure AD administrative unit (relevant only when azure-user-portal-access=true) | [optional]
**azure_client_id** | Option<**String**> | Azure Client ID | [optional]
**azure_client_secret** | Option<**String**> | Azure Client Secret | [optional]
**azure_tenant_id** | Option<**String**> | Azure Tenant ID | [optional]
**custom_username_template** | Option<**String**> | Customize how temporary usernames are generated using go template | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**fixed_user_claim_keyname** | Option<**String**> | FixedUserClaimKeyname | [optional][default to false]
**fixed_user_only** | Option<**bool**> | Fixed user | [optional][default to false]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Dynamic secret name | 
**password_length** | Option<**String**> | The length of the password to be generated | [optional]
**producer_encryption_key_name** | Option<**String**> | Dynamic secret encryption key | [optional]
**secure_access_enable** | Option<**String**> | Enable/Disable secure remote access [true/false] | [optional]
**secure_access_url** | Option<**String**> | Destination URL to inject secrets | [optional]
**secure_access_web** | Option<**bool**> | Enable Web Secure Remote Access | [optional][default to true]
**secure_access_web_browsing** | Option<**bool**> | Secure browser via Akeyless's Secure Remote Access (SRA) | [optional][default to false]
**secure_access_web_proxy** | Option<**bool**> | Web-Proxy via Akeyless's Secure Remote Access (SRA) | [optional][default to false]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**target_name** | Option<**String**> | Target name | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**user_group_obj_id** | Option<**String**> | User Group Object Id | [optional]
**user_portal_access** | Option<**bool**> | Azure User portal access | [optional][default to false]
**user_principal_name** | Option<**String**> | User Principal Name | [optional]
**user_programmatic_access** | Option<**bool**> | Azure User programmatic access | [optional][default to false]
**user_role_template_id** | Option<**String**> | User Role Template Id | [optional]
**user_ttl** | Option<**String**> | User TTL | [optional][default to 60m]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


