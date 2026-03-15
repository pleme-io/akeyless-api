# TargetCreateSsh

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Description of the object | [optional]
**host** | Option<**String**> | SSH host name | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key** | Option<**String**> | The name of a key that used to encrypt the target secret value (if empty, the account default protectionKey key will be used) | [optional]
**max_versions** | Option<**String**> | Set the maximum number of versions, limited by the account settings defaults. | [optional]
**name** | **String** | Target name | 
**port** | Option<**String**> | SSH port | [optional][default to 22]
**private_key** | Option<**String**> | SSH private key | [optional]
**private_key_password** | Option<**String**> | SSH private key password | [optional]
**ssh_password** | Option<**String**> | SSH password to rotate | [optional]
**ssh_username** | Option<**String**> | SSH username | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


