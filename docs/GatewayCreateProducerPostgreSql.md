# GatewayCreateProducerPostgreSql

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**creation_statements** | Option<**String**> | PostgreSQL Creation statements | [optional]
**custom_username_template** | Option<**String**> | Customize how temporary usernames are generated using go template | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Dynamic secret name | 
**password_length** | Option<**String**> | The length of the password to be generated | [optional]
**postgresql_db_name** | Option<**String**> | PostgreSQL DB Name | [optional]
**postgresql_host** | Option<**String**> | PostgreSQL Host | [optional][default to 127.0.0.1]
**postgresql_password** | Option<**String**> | PostgreSQL Password | [optional]
**postgresql_port** | Option<**String**> | PostgreSQL Port | [optional][default to 5432]
**postgresql_username** | Option<**String**> | PostgreSQL Username | [optional]
**producer_encryption_key** | Option<**String**> | Dynamic producer encryption key | [optional]
**revocation_statement** | Option<**String**> | PostgreSQL Revocation statements | [optional]
**secure_access_bastion_issuer** | Option<**String**> | Deprecated. use secure-access-certificate-issuer | [optional]
**secure_access_certificate_issuer** | Option<**String**> | Path to the SSH Certificate Issuer for your Akeyless Secure Access | [optional]
**secure_access_db_name** | Option<**String**> | The DB name (relevant only for DB Dynamic-Secret) | [optional]
**secure_access_db_schema** | Option<**String**> | The DB schema | [optional]
**secure_access_delay** | Option<**i64**> | The delay duration, in seconds, to wait after generating just-in-time credentials. Accepted range: 0-120 seconds | [optional]
**secure_access_enable** | Option<**String**> | Enable/Disable secure remote access [true/false] | [optional]
**secure_access_host** | Option<**Vec<String>**> | Target DB servers for connections (In case of Linked Target association, host(s) will inherit Linked Target hosts) | [optional]
**secure_access_web** | Option<**bool**> | Enable Web Secure Remote Access | [optional][default to false]
**ssl** | Option<**bool**> | Enable/Disable SSL [true/false] | [optional][default to false]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**target_name** | Option<**String**> | Target name | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**user_ttl** | Option<**String**> | User TTL | [optional][default to 60m]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


