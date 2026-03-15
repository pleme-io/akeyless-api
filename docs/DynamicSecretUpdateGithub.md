# DynamicSecretUpdateGithub

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**github_app_id** | Option<**i64**> | Github app id | [optional]
**github_app_private_key** | Option<**String**> | App private key | [optional]
**github_base_url** | Option<**String**> | Base URL | [optional][default to https://api.github.com/]
**installation_id** | Option<**i64**> | GitHub application installation id | [optional]
**installation_organization** | Option<**String**> | Optional, mutually exclusive with installation id, GitHub organization name | [optional]
**installation_repository** | Option<**String**> | Optional, mutually exclusive with installation id, GitHub repository '<owner>/<repo-name>' | [optional]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Dynamic secret name | 
**new_name** | Option<**String**> | Dynamic secret name | [optional]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**target_name** | Option<**String**> | Target name | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**token_permissions** | Option<**Vec<String>**> | Optional - installation token's allowed permissions | [optional]
**token_repositories** | Option<**Vec<String>**> | Optional - installation token's allowed repositories | [optional]
**token_ttl** | Option<**String**> | Token TTL | [optional][default to 60m]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


