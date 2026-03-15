# CreateGitlabTarget

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**comment** | Option<**String**> | Deprecated - use description | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**gitlab_access_token** | Option<**String**> | Gitlab access token | [optional]
**gitlab_certificate** | Option<**String**> | Gitlab tls certificate (base64 encoded) | [optional]
**gitlab_url** | Option<**String**> | Gitlab base url | [optional][default to https://gitlab.com/]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key** | Option<**String**> | The name of a key that used to encrypt the target secret value (if empty, the account default protectionKey key will be used) | [optional]
**max_versions** | Option<**String**> | Set the maximum number of versions, limited by the account settings defaults. | [optional]
**name** | **String** | Target name | 
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


