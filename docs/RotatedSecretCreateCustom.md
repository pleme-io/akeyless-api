# RotatedSecretCreateCustom

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authentication_credentials** | Option<**String**> | The credentials to connect with use-user-creds/use-target-creds | [optional][default to use-user-creds]
**auto_rotate** | Option<**String**> | Whether to automatically rotate every --rotation-interval days, or disable existing automatic rotation [true/false] | [optional]
**custom_payload** | Option<**String**> | Secret payload to be sent with rotation request | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**enable_password_policy** | Option<**String**> | Enable password policy | [optional]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key** | Option<**String**> | The name of a key that used to encrypt the secret value (if empty, the account default protectionKey key will be used) | [optional]
**max_versions** | Option<**String**> | Set the maximum number of versions, limited by the account settings defaults. | [optional]
**name** | **String** | Rotated secret name | 
**password_length** | Option<**String**> | The length of the password to be generated | [optional]
**rotate_after_disconnect** | Option<**String**> | Rotate the value of the secret after SRA session ends [true/false] | [optional][default to false]
**rotation_event_in** | Option<**Vec<String>**> | How many days before the rotation of the item would you like to be notified | [optional]
**rotation_hour** | Option<**i32**> | The Hour of the rotation in UTC | [optional]
**rotation_interval** | Option<**String**> | The number of days to wait between every automatic key rotation (1-365) | [optional]
**secure_access_allow_external_user** | Option<**bool**> | Allow providing external user for a domain users | [optional][default to false]
**secure_access_bastion_issuer** | Option<**String**> | Deprecated. use secure-access-certificate-issuer | [optional]
**secure_access_certificate_issuer** | Option<**String**> | Path to the SSH Certificate Issuer for your Akeyless Secure Access | [optional]
**secure_access_enable** | Option<**String**> | Enable/Disable secure remote access [true/false] | [optional]
**secure_access_host** | Option<**Vec<String>**> | Target servers for connections (In case of Linked Target association, host(s) will inherit Linked Target hosts - Relevant only for Dynamic Secrets/producers) | [optional]
**secure_access_rdp_domain** | Option<**String**> | Default domain name server. i.e. microsoft.com | [optional]
**secure_access_rdp_user** | Option<**String**> | Override the RDP Domain username | [optional]
**secure_access_ssh_user** | Option<**String**> | Override the SSH username as indicated in SSH Certificate Issuer | [optional]
**secure_access_url** | Option<**String**> | Destination URL to inject secrets | [optional]
**secure_access_web** | Option<**bool**> | Enable Web Secure Remote Access | [optional][default to false]
**secure_access_web_browsing** | Option<**bool**> | Secure browser via Akeyless's Secure Remote Access (SRA) | [optional][default to false]
**secure_access_web_proxy** | Option<**bool**> | Web-Proxy via Akeyless's Secure Remote Access (SRA) | [optional][default to false]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**target_name** | **String** | The target name to associate | 
**timeout_sec** | Option<**i64**> | Maximum allowed time in seconds for the custom rotator to return the results | [optional][default to 40]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**use_capital_letters** | Option<**String**> | Password must contain capital letters [true/false] | [optional]
**use_lower_letters** | Option<**String**> | Password must contain lower case letters [true/false] | [optional]
**use_numbers** | Option<**String**> | Password must contain numbers [true/false] | [optional]
**use_special_characters** | Option<**String**> | Password must contain special characters [true/false] | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


