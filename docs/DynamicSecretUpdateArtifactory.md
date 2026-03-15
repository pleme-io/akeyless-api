# DynamicSecretUpdateArtifactory

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**artifactory_admin_name** | Option<**String**> | Artifactory Admin Name | [optional]
**artifactory_admin_pwd** | Option<**String**> | Artifactory Admin password | [optional]
**artifactory_token_audience** | **String** | Token Audience | 
**artifactory_token_scope** | **String** | Token Scope | 
**base_url** | Option<**String**> | Base URL | [optional]
**custom_username_template** | Option<**String**> | Customize how temporary usernames are generated using go template | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Dynamic secret name | 
**new_name** | Option<**String**> | Dynamic secret name | [optional]
**producer_encryption_key_name** | Option<**String**> | Dynamic producer encryption key | [optional]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**target_name** | Option<**String**> | Target name | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**user_ttl** | Option<**String**> | User TTL | [optional][default to 60m]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


