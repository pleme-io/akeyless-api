# GatewayCreateProducerCassandra

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cassandra_creation_statements** | Option<**String**> | Cassandra creation statements | [optional]
**cassandra_hosts** | Option<**String**> | Cassandra hosts IP or addresses, comma separated | [optional]
**cassandra_password** | Option<**String**> | Cassandra superuser password | [optional]
**cassandra_port** | Option<**String**> | Cassandra port | [optional][default to 9042]
**cassandra_username** | Option<**String**> | Cassandra superuser username | [optional]
**custom_username_template** | Option<**String**> | Customize how temporary usernames are generated using go template | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Dynamic secret name | 
**password_length** | Option<**String**> | The length of the password to be generated | [optional]
**producer_encryption_key_name** | Option<**String**> | Dynamic producer encryption key | [optional]
**ssl** | Option<**bool**> | Enable/Disable SSL [true/false] | [optional][default to false]
**ssl_certificate** | Option<**String**> | SSL CA certificate in base64 encoding generated from a trusted Certificate Authority (CA) | [optional]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**target_name** | Option<**String**> | Target name | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**user_ttl** | Option<**String**> | User TTL | [optional][default to 60m]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


