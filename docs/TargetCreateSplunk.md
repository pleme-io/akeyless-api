# TargetCreateSplunk

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**audience** | Option<**String**> | Splunk token audience (required when using token authentication for rotation) | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key** | Option<**String**> | The name of a key that used to encrypt the target secret value (if empty, the account default protectionKey key will be used) | [optional]
**max_versions** | Option<**String**> | Set the maximum number of versions, limited by the account settings defaults. | [optional]
**name** | **String** | Target name | 
**password** | Option<**String**> | Splunk Password (used when authenticating with username/password) | [optional]
**token** | Option<**String**> | Splunk Token (used when authenticating with token) | [optional]
**token_owner** | Option<**String**> | Splunk Token Owner (required when using token authentication for rotation) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**url** | **String** | Splunk server URL | 
**use_tls** | Option<**bool**> | Use TLS certificate verification when connecting to the Splunk management API | [optional][default to true]
**username** | Option<**String**> | Splunk Username (used when authenticating with username/password) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


