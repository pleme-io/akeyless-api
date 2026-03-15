# UpdateRdpTargetDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**admin_name** | Option<**String**> | The admin name | [optional]
**admin_pwd** | Option<**String**> | The admin password | [optional]
**host_name** | Option<**String**> | The rdp host name | [optional]
**host_port** | Option<**String**> | The rdp port | [optional][default to 22]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**keep_prev_version** | Option<**String**> | Whether to keep previous version [true/false]. If not set, use default according to account settings | [optional]
**name** | **String** | Target name | 
**new_version** | Option<**bool**> | Deprecated | [optional]
**protection_key** | Option<**String**> | The name of a key that used to encrypt the target secret value (if empty, the account default protectionKey key will be used) | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


