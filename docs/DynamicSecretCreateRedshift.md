# DynamicSecretCreateRedshift

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**creation_statements** | Option<**String**> | Redshift Creation statements | [optional]
**custom_username_template** | Option<**String**> | Customize how temporary usernames are generated using go template | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Dynamic secret name | 
**password_length** | Option<**String**> | The length of the password to be generated | [optional]
**producer_encryption_key** | Option<**String**> | Dynamic producer encryption key | [optional]
**redshift_db_name** | Option<**String**> | Redshift DB Name | [optional]
**redshift_host** | Option<**String**> | Redshift Host | [optional][default to 127.0.0.1]
**redshift_password** | Option<**String**> | Redshift Password | [optional]
**redshift_port** | Option<**String**> | Redshift Port | [optional][default to 5439]
**redshift_username** | Option<**String**> | Redshift Username | [optional]
**secure_access_enable** | Option<**String**> | Enable/Disable secure remote access [true/false] | [optional]
**secure_access_host** | Option<**Vec<String>**> | Target DB servers for connections (In case of Linked Target association, host(s) will inherit Linked Target hosts) | [optional]
**ssl** | Option<**bool**> | Enable/Disable SSL [true/false] | [optional][default to false]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**target_name** | Option<**String**> | Target name | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**user_ttl** | Option<**String**> | User TTL | [optional][default to 60m]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


