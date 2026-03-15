# TargetUpdateOpenAi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_key** | Option<**String**> | API key for OpenAI | [optional]
**api_key_id** | Option<**String**> | API key ID | [optional]
**description** | Option<**String**> | Description of the object | [optional][default to default_comment]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**keep_prev_version** | Option<**String**> | Whether to keep previous version [true/false]. If not set, use default according to account settings | [optional]
**key** | Option<**String**> | The name of a key that used to encrypt the target secret value (if empty, the account default protectionKey key will be used) | [optional]
**max_versions** | Option<**String**> | Set the maximum number of versions, limited by the account settings defaults. | [optional]
**model** | Option<**String**> | Default model to use with OpenAI | [optional]
**name** | **String** | Target name | 
**new_comment** | Option<**String**> | Deprecated - use description | [optional][default to default_comment]
**new_name** | Option<**String**> | New target name | [optional]
**openai_url** | Option<**String**> | Base URL of the OpenAI API | [optional][default to https://api.openai.com/v1]
**organization_id** | Option<**String**> | Organization ID | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


