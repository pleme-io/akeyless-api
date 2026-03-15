# CreatePingTarget

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**administrative_port** | Option<**String**> | Ping Federate administrative port | [optional][default to 9999]
**authorization_port** | Option<**String**> | Ping Federate authorization port | [optional][default to 9031]
**comment** | Option<**String**> | Deprecated - use description | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key** | Option<**String**> | The name of a key that used to encrypt the target secret value (if empty, the account default protectionKey key will be used) | [optional]
**max_versions** | Option<**String**> | Set the maximum number of versions, limited by the account settings defaults. | [optional]
**name** | **String** | Target name | 
**password** | Option<**String**> | Ping Federate privileged user password | [optional]
**ping_url** | Option<**String**> | Ping URL | [optional]
**privileged_user** | Option<**String**> | Ping Federate privileged user | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


