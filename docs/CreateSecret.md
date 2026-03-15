# CreateSecret

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accessibility** | Option<**String**> | for personal password manager | [optional][default to regular]
**change_event** | Option<**String**> | Trigger an event when a secret value changed [true/false] (Relevant only for Static Secret) | [optional]
**custom_field** | Option<**std::collections::HashMap<String, String>**> | For Password Management use, additional fields | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**format** | Option<**String**> | Secret format [text/json/key-value] (relevant only for type 'generic') | [optional][default to text]
**inject_url** | Option<**Vec<String>**> | For Password Management use, reflect the website context | [optional]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**max_versions** | Option<**String**> | Set the maximum number of versions, limited by the account settings defaults. | [optional]
**metadata** | Option<**String**> | Deprecated - use description | [optional]
**multiline_value** | Option<**bool**> | The provided value is a multiline value (separated by '\\n') | [optional]
**name** | **String** | Secret name | 
**password** | Option<**String**> | For Password Management use, additional fields | [optional]
**protection_key** | Option<**String**> | The name of a key that used to encrypt the secret value (if empty, the account default protectionKey key will be used) | [optional]
**secure_access_bastion_issuer** | Option<**String**> | Deprecated. use secure-access-certificate-issuer | [optional]
**secure_access_certificate_issuer** | Option<**String**> | Path to the SSH Certificate Issuer for your Akeyless Secure Access | [optional]
**secure_access_enable** | Option<**String**> | Enable/Disable secure remote access [true/false] | [optional]
**secure_access_gateway** | Option<**String**> |  | [optional]
**secure_access_host** | Option<**Vec<String>**> | Target servers for connections (In case of Linked Target association, host(s) will inherit Linked Target hosts - Relevant only for Dynamic Secrets/producers) | [optional]
**secure_access_rdp_user** | Option<**String**> | Remote Desktop Username | [optional]
**secure_access_ssh_creds** | Option<**String**> | Static-Secret values contains SSH Credentials, either Private Key or Password [password/private-key] | [optional]
**secure_access_ssh_user** | Option<**String**> | Override the SSH username as indicated in SSH Certificate Issuer | [optional]
**secure_access_url** | Option<**String**> | Destination URL to inject secrets | [optional]
**secure_access_web_browsing** | Option<**bool**> | Secure browser via Akeyless's Secure Remote Access (SRA) | [optional][default to false]
**secure_access_web_proxy** | Option<**bool**> | Web-Proxy via Akeyless's Secure Remote Access (SRA) | [optional][default to false]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**r#type** | Option<**String**> | The secret sub type [generic/password] | [optional][default to generic]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**username** | Option<**String**> | For Password Management use | [optional]
**value** | **String** | The secret value (relevant only for type 'generic') | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


