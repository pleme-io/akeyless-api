# GatewayCreateProducerMongo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**custom_username_template** | Option<**String**> | Customize how temporary usernames are generated using go template | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**mongodb_atlas_api_private_key** | Option<**String**> | MongoDB Atlas private key | [optional]
**mongodb_atlas_api_public_key** | Option<**String**> | MongoDB Atlas public key | [optional]
**mongodb_atlas_project_id** | Option<**String**> | MongoDB Atlas project ID | [optional]
**mongodb_custom_data** | Option<**String**> | MongoDB custom data | [optional]
**mongodb_default_auth_db** | Option<**String**> | MongoDB server default authentication database | [optional]
**mongodb_host_port** | Option<**String**> | MongoDB server host and port | [optional]
**mongodb_name** | Option<**String**> | MongoDB Name | [optional]
**mongodb_password** | Option<**String**> | MongoDB server password. You will prompted to provide a password if it will not appear in CLI parameters | [optional]
**mongodb_roles** | Option<**String**> | MongoDB Roles | [optional][default to []]
**mongodb_scopes** | Option<**String**> | MongoDB Scopes (Atlas only) | [optional]
**mongodb_server_uri** | Option<**String**> | MongoDB server URI | [optional]
**mongodb_uri_options** | Option<**String**> | MongoDB server URI options | [optional]
**mongodb_username** | Option<**String**> | MongoDB server username | [optional]
**name** | **String** | Dynamic secret name | 
**password_length** | Option<**String**> | The length of the password to be generated | [optional]
**producer_encryption_key_name** | Option<**String**> | Encrypt producer with following key | [optional]
**secure_access_bastion_issuer** | Option<**String**> | Deprecated. use secure-access-certificate-issuer | [optional]
**secure_access_certificate_issuer** | Option<**String**> | Path to the SSH Certificate Issuer for your Akeyless Secure Access | [optional]
**secure_access_db_name** | Option<**String**> | The DB name (relevant only for DB Dynamic-Secret) | [optional]
**secure_access_delay** | Option<**i64**> | The delay duration, in seconds, to wait after generating just-in-time credentials. Accepted range: 0-120 seconds | [optional]
**secure_access_enable** | Option<**String**> | Enable/Disable secure remote access [true/false] | [optional]
**secure_access_host** | Option<**Vec<String>**> | Target DB servers for connections (In case of Linked Target association, host(s) will inherit Linked Target hosts) | [optional]
**secure_access_web** | Option<**bool**> | Enable Web Secure Remote Access | [optional][default to false]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**target_name** | Option<**String**> | Target name | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**user_ttl** | Option<**String**> | User TTL | [optional][default to 60m]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


