# DynamicSecretUpdateOracleDb

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**custom_username_template** | Option<**String**> | Customize how temporary usernames are generated using go template | [optional]
**db_server_certificates** | Option<**String**> | (Optional) DB server certificates | [optional]
**db_server_name** | Option<**String**> | (Optional) Server name for certificate verification | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Dynamic secret name | 
**new_name** | Option<**String**> | Dynamic secret name | [optional]
**oracle_host** | Option<**String**> | Oracle Host | [optional][default to 127.0.0.1]
**oracle_password** | Option<**String**> | Oracle Password | [optional]
**oracle_port** | Option<**String**> | Oracle Port | [optional][default to 1521]
**oracle_revocation_statements** | Option<**String**> | Oracle Revocation statements | [optional]
**oracle_screation_statements** | Option<**String**> | Oracle Creation statements | [optional]
**oracle_service_name** | Option<**String**> | Oracle DB Name | [optional]
**oracle_username** | Option<**String**> | Oracle Username | [optional]
**password_length** | Option<**String**> | The length of the password to be generated | [optional]
**producer_encryption_key_name** | Option<**String**> | Dynamic producer encryption key | [optional]
**secure_access_bastion_issuer** | Option<**String**> | Deprecated. use secure-access-certificate-issuer | [optional]
**secure_access_certificate_issuer** | Option<**String**> | Path to the SSH Certificate Issuer for your Akeyless Secure Access | [optional]
**secure_access_enable** | Option<**String**> | Enable/Disable secure remote access [true/false] | [optional][default to false]
**secure_access_host** | Option<**Vec<String>**> | Target DB servers for connections (In case of Linked Target association, host(s) will inherit Linked Target hosts) | [optional]
**secure_access_web** | Option<**bool**> | Enable Web Secure Remote Access | [optional][default to false]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**target_name** | Option<**String**> | Target name | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**user_ttl** | Option<**String**> | User TTL | [optional][default to 60m]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


