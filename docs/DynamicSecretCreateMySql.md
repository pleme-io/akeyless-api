# DynamicSecretCreateMySql

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
**mysql_dbname** | Option<**String**> | MySQL DB Name | [optional]
**mysql_host** | Option<**String**> | MySQL Host | [optional][default to 127.0.0.1]
**mysql_password** | Option<**String**> | MySQL Password | [optional]
**mysql_port** | Option<**String**> | MySQL Port | [optional][default to 3306]
**mysql_revocation_statements** | Option<**String**> | MySQL Revocation statements | [optional]
**mysql_screation_statements** | Option<**String**> | MySQL Creation statements | [optional]
**mysql_username** | Option<**String**> | MySQL Username | [optional]
**name** | **String** | Dynamic secret name | 
**password_length** | Option<**String**> | The length of the password to be generated | [optional]
**producer_encryption_key_name** | Option<**String**> | Dynamic producer encryption key | [optional]
**secure_access_bastion_issuer** | Option<**String**> | Deprecated. use secure-access-certificate-issuer | [optional]
**secure_access_certificate_issuer** | Option<**String**> | Path to the SSH Certificate Issuer for your Akeyless Secure Access | [optional]
**secure_access_db_name** | Option<**String**> | The DB name (relevant only for DB Dynamic-Secret) | [optional]
**secure_access_delay** | Option<**i64**> | The delay duration, in seconds, to wait after generating just-in-time credentials. Accepted range: 0-120 seconds | [optional]
**secure_access_enable** | Option<**String**> | Enable/Disable secure remote access [true/false] | [optional]
**secure_access_host** | Option<**Vec<String>**> | Target DB servers for connections (In case of Linked Target association, host(s) will inherit Linked Target hosts) | [optional]
**secure_access_web** | Option<**bool**> | Enable Web Secure Remote Access | [optional][default to false]
**ssl** | Option<**bool**> | Enable/Disable SSL [true/false] | [optional][default to false]
**ssl_certificate** | Option<**String**> | SSL connection certificate | [optional]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**target_name** | Option<**String**> | Target name | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**user_ttl** | Option<**String**> | User TTL | [optional][default to 60m]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


