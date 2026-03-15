# UpdateSshTargetDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**host** | Option<**String**> | The ssh host name | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**keep_prev_version** | Option<**String**> | Whether to keep previous version [true/false]. If not set, use default according to account settings | [optional]
**name** | **String** | Target name | 
**new_version** | Option<**bool**> | Deprecated | [optional]
**port** | Option<**String**> | ssh port | [optional][default to 22]
**private_key** | Option<**String**> | ssh private key | [optional]
**private_key_password** | Option<**String**> | The ssh private key password | [optional]
**protection_key** | Option<**String**> | The name of a key that used to encrypt the target secret value (if empty, the account default protectionKey key will be used) | [optional]
**ssh_password** | Option<**String**> | ssh pawssword to rotate | [optional]
**ssh_username** | Option<**String**> | ssh username | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


