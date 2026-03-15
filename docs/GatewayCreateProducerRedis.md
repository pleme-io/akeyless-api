# GatewayCreateProducerRedis

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acl_rules** | Option<**String**> | A JSON array list of redis ACL rules to attach to the created user. For available rules see the ACL CAT command https://redis.io/commands/acl-cat By default the user will have permissions to read all keys '[\"~*\", \"+@read\"]' | [optional]
**custom_username_template** | Option<**String**> | Customize how temporary usernames are generated using go template | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**host** | Option<**String**> | Redis Host | [optional][default to 127.0.0.1]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Dynamic secret name | 
**password** | Option<**String**> | Redis Password | [optional]
**password_length** | Option<**String**> | The length of the password to be generated | [optional]
**port** | Option<**String**> | Redis Port | [optional][default to 6379]
**producer_encryption_key_name** | Option<**String**> | Dynamic producer encryption key | [optional]
**ssl** | Option<**bool**> | Enable/Disable SSL [true/false] | [optional][default to false]
**ssl_certificate** | Option<**String**> | SSL CA certificate in base64 encoding generated from a trusted Certificate Authority (CA) | [optional]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**target_name** | Option<**String**> | Target name | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**user_ttl** | Option<**String**> | User TTL | [optional][default to 60m]
**username** | Option<**String**> | Redis Username | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


