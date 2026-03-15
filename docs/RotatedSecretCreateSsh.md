# RotatedSecretCreateSsh

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
**key_data_base64** | Option<**String**> | Private key file contents encoded using base64 | [optional]
**max_versions** | Option<**String**> | Set the maximum number of versions, limited by the account settings defaults. | [optional]
**name** | **String** | Rotated secret name | 
**password_length** | Option<**String**> | The length of the password to be generated | [optional]
**public_key_remote_path** | Option<**String**> | The path to the public key that will be rotated on the server | [optional]
**rotate_after_disconnect** | Option<**String**> | Rotate the value of the secret after SRA session ends [true/false] | [optional][default to false]
**rotated_password** | Option<**String**> | rotated-username password (relevant only for rotator-type=password) | [optional]
**rotated_username** | Option<**String**> | username to be rotated, if selected use-self-creds at rotator-creds-type, this username will try to rotate it's own password, if use-target-creds is selected, target credentials will be use to rotate the rotated-password (relevant only for rotator-type=password) | [optional]
**rotation_event_in** | Option<**Vec<String>**> | How many days before the rotation of the item would you like to be notified | [optional]
**rotation_hour** | Option<**i32**> | The Hour of the rotation in UTC | [optional]
**rotation_interval** | Option<**String**> | The number of days to wait between every automatic key rotation (1-365) | [optional]
**rotator_custom_cmd** | Option<**String**> | Custom rotation command | [optional]
**rotator_type** | **String** | The rotator type. options: [target/password/key] | 
**same_password** | Option<**String**> | Rotate same password for each host from the Linked Target (relevant only for Linked Target) | [optional]
**secure_access_allow_external_user** | Option<**bool**> | Allow providing external user for a domain users | [optional][default to false]
**secure_access_bastion_issuer** | Option<**String**> | Deprecated. use secure-access-certificate-issuer | [optional]
**secure_access_certificate_issuer** | Option<**String**> | Path to the SSH Certificate Issuer for your Akeyless Secure Access | [optional]
**secure_access_enable** | Option<**String**> | Enable/Disable secure remote access [true/false] | [optional]
**secure_access_host** | Option<**Vec<String>**> | Target servers for connections (In case of Linked Target association, host(s) will inherit Linked Target hosts - Relevant only for Dynamic Secrets/producers) | [optional]
**secure_access_rdp_domain** | Option<**String**> | Default domain name server. i.e. microsoft.com | [optional]
**secure_access_rdp_user** | Option<**String**> | Override the RDP Domain username | [optional]
**secure_access_ssh_user** | Option<**String**> | Override the SSH username as indicated in SSH Certificate Issuer | [optional]
**secure_access_target_type** | Option<**String**> | Specify target type. Options are ssh or rdp | [optional][default to false]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**target_name** | **String** | The target name to associate | 
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


