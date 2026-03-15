# DynamicSecretUpdateGitlab

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**gitlab_access_token** | Option<**String**> | Gitlab access token | [optional]
**gitlab_access_type** | **String** | Gitlab access token type [project,group] | 
**gitlab_certificate** | Option<**String**> | Gitlab tls certificate (base64 encoded) | [optional]
**gitlab_role** | Option<**String**> | Gitlab role | [optional]
**gitlab_token_scopes** | **String** | Comma-separated list of access token scopes to grant | 
**gitlab_url** | Option<**String**> | Gitlab base url | [optional][default to https://gitlab.com/]
**group_name** | Option<**String**> | Gitlab group name, required for access-type=group | [optional]
**installation_organization** | Option<**String**> | Gitlab project name, required for access-type=project | [optional]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Dynamic secret name | 
**new_name** | Option<**String**> | Dynamic secret name | [optional]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**target_name** | Option<**String**> | Target name | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**ttl** | Option<**String**> | Access Token TTL | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


