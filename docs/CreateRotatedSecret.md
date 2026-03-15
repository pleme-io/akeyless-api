# CreateRotatedSecret

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider_type** | Option<**String**> |  | [optional]
**api_id** | Option<**String**> | API ID to rotate (relevant only for rotator-type=api-key) | [optional]
**api_key** | Option<**String**> | API key to rotate (relevant only for rotator-type=api-key) | [optional]
**application_id** | Option<**String**> | ApplicationId (used in azure) | [optional]
**authentication_credentials** | Option<**String**> | The credentials to connect with use-user-creds/use-target-creds | [optional][default to use-user-creds]
**auto_rotate** | Option<**String**> | Whether to automatically rotate every --rotation-interval days, or disable existing automatic rotation [true/false] | [optional]
**aws_region** | Option<**String**> | Aws Region (relevant only for aws) | [optional][default to us-east-2]
**custom_payload** | Option<**String**> | Secret payload to be sent with rotation request (relevant only for rotator-type=custom) | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**gcp_key** | Option<**String**> | Base64-encoded service account private key text | [optional]
**gcp_service_account_email** | Option<**String**> | The email of the gcp service account to rotate | [optional]
**gcp_service_account_key_id** | Option<**String**> | The key id of the gcp service account to rotate | [optional]
**grace_rotation** | Option<**String**> | Create a new access key without deleting the old key from AWS for backup (relevant only for AWS) [true/false] | [optional]
**host_provider** | Option<**String**> | Host provider type [explicit/target], Default Host provider is explicit, Relevant only for Secure Remote Access of ssh cert issuer, ldap rotated secret and ldap dynamic secret | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key** | Option<**String**> | The name of a key that used to encrypt the secret value (if empty, the account default protectionKey key will be used) | [optional]
**metadata** | Option<**String**> | Deprecated - use description | [optional]
**name** | **String** | Secret name | 
**password_length** | Option<**String**> | The length of the password to be generated | [optional]
**rotate_after_disconnect** | Option<**String**> | Rotate the value of the secret after SRA session ends [true/false] | [optional][default to false]
**rotated_password** | Option<**String**> | rotated-username password (relevant only for rotator-type=password) | [optional]
**rotated_username** | Option<**String**> | username to be rotated, if selected use-self-creds at rotator-creds-type, this username will try to rotate it's own password, if use-target-creds is selected, target credentials will be use to rotate the rotated-password (relevant only for rotator-type=password) | [optional]
**rotation_hour** | Option<**i32**> | The Hour of the rotation in UTC. Default rotation-hour is 14:00 | [optional]
**rotation_interval** | Option<**String**> | The number of days to wait between every automatic key rotation (1-365) | [optional]
**rotator_creds_type** | Option<**String**> |  | [optional]
**rotator_custom_cmd** | Option<**String**> | Custom rotation command (relevant only for ssh target) | [optional]
**rotator_type** | **String** | Rotator Type | 
**same_password** | Option<**String**> | Rotate same password for each host from the Linked Target (relevant only for Linked Target) | [optional]
**secure_access_allow_external_user** | Option<**bool**> | Allow providing external user for a domain users (relevant only for rdp) | [optional][default to false]
**secure_access_aws_account_id** | Option<**String**> | The AWS account id (relevant only for aws) | [optional]
**secure_access_aws_native_cli** | Option<**bool**> | The AWS native cli | [optional]
**secure_access_bastion_issuer** | Option<**String**> | Deprecated. use secure-access-certificate-issuer | [optional]
**secure_access_certificate_issuer** | Option<**String**> | Path to the SSH Certificate Issuer for your Akeyless Secure Access | [optional]
**secure_access_db_name** | Option<**String**> | The DB name (relevant only for DB Dynamic-Secret) | [optional]
**secure_access_db_schema** | Option<**String**> | The db schema (relevant only for mssql or postgresql) | [optional]
**secure_access_disable_concurrent_connections** | Option<**bool**> | Enable this flag to prevent simultaneous use of the same secret | [optional]
**secure_access_enable** | Option<**String**> | Enable/Disable secure remote access [true/false] | [optional]
**secure_access_host** | Option<**Vec<String>**> | Target servers for connections (In case of Linked Target association, host(s) will inherit Linked Target hosts - Relevant only for Dynamic Secrets/producers) | [optional]
**secure_access_rdp_domain** | Option<**String**> | Required when the Dynamic Secret is used for a domain user (relevant only for RDP Dynamic-Secret) | [optional]
**secure_access_rdp_user** | Option<**String**> | Override the RDP Domain username (relevant only for rdp) | [optional]
**secure_access_url** | Option<**String**> | Destination URL to inject secrets | [optional]
**secure_access_web** | Option<**bool**> | Enable Web Secure Remote Access | [optional][default to false]
**secure_access_web_browsing** | Option<**bool**> | Secure browser viaAkeyless's Secure Remote Access (SRA) (relevant only for aws or azure) | [optional][default to false]
**secure_access_web_proxy** | Option<**bool**> | Web-Proxy via Akeyless's Secure Remote Access (SRA) (relevant only for aws or azure) | [optional][default to false]
**ssh_password** | Option<**String**> | Deprecated: use RotatedPassword | [optional]
**ssh_username** | Option<**String**> | Deprecated: use RotatedUser | [optional]
**storage_account_key_name** | Option<**String**> | The name of the storage account key to rotate [key1/key2/kerb1/kerb2] (relevat to azure-storage-account) | [optional]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**target** | Option<**Vec<String>**> | A list of linked targets to be associated, Relevant only for Secure Remote Access for ssh cert issuer, ldap rotated secret and ldap dynamic secret, To specify multiple targets use argument multiple times | [optional]
**target_name** | **String** | Target name | 
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**user_attribute** | Option<**String**> | LDAP User Attribute, Default value \"cn\" | [optional][default to cn]
**user_dn** | Option<**String**> | LDAP User Base DN | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


