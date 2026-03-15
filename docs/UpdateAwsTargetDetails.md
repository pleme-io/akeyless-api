# UpdateAwsTargetDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_key** | Option<**String**> | The aws secret access key | [optional]
**access_key_id** | Option<**String**> | The aws access key id | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**keep_prev_version** | Option<**String**> | Whether to keep previous version [true/false]. If not set, use default according to account settings | [optional]
**name** | **String** | Target name | 
**new_version** | Option<**bool**> | Deprecated | [optional]
**protection_key** | Option<**String**> | The name of a key that used to encrypt the target secret value (if empty, the account default protectionKey key will be used) | [optional]
**region** | Option<**String**> | The aws region | [optional][default to us-east-2]
**session_token** | Option<**String**> | Required only for temporary security credentials retrieved via STS, otherwise it can be an empty string | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


