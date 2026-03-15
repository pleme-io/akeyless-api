# DynamicSecretUpdateRabbitMq

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Dynamic secret name | 
**new_name** | Option<**String**> | Dynamic secret name | [optional]
**password_length** | Option<**String**> | The length of the password to be generated | [optional]
**producer_encryption_key_name** | Option<**String**> | Dynamic producer encryption key | [optional]
**rabbitmq_admin_pwd** | Option<**String**> | RabbitMQ Admin password | [optional]
**rabbitmq_admin_user** | Option<**String**> | RabbitMQ Admin User | [optional]
**rabbitmq_server_uri** | Option<**String**> | Server URI | [optional]
**rabbitmq_user_conf_permission** | Option<**String**> | User configuration permission | [optional]
**rabbitmq_user_read_permission** | Option<**String**> | User read permission | [optional]
**rabbitmq_user_tags** | Option<**String**> | User Tags | [optional]
**rabbitmq_user_vhost** | Option<**String**> | User Virtual Host | [optional]
**rabbitmq_user_write_permission** | Option<**String**> | User write permission | [optional]
**secure_access_enable** | Option<**String**> | Enable/Disable secure remote access [true/false] | [optional]
**secure_access_url** | Option<**String**> | Destination URL to inject secrets | [optional]
**secure_access_web** | Option<**bool**> | Enable Web Secure Remote Access | [optional][default to true]
**secure_access_web_browsing** | Option<**bool**> | Secure browser via Akeyless's Secure Remote Access (SRA) | [optional][default to false]
**secure_access_web_proxy** | Option<**bool**> | Web-Proxy via Akeyless's Secure Remote Access (SRA) | [optional][default to false]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**target_name** | Option<**String**> | Target name | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**user_ttl** | Option<**String**> | User TTL | [optional][default to 60m]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


