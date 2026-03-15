# UpdateDbTargetDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**db_type** | Option<**String**> |  | [optional]
**host** | Option<**String**> |  | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**keep_prev_version** | Option<**String**> | Whether to keep previous version [true/false]. If not set, use default according to account settings | [optional]
**mongo_db_name** | Option<**String**> |  | [optional]
**mongo_uri** | Option<**String**> |  | [optional]
**name** | **String** | Target name | 
**new_version** | Option<**bool**> | Deprecated | [optional]
**port** | Option<**String**> |  | [optional]
**protection_key** | Option<**String**> | The name of a key that used to encrypt the target secret value (if empty, the account default protectionKey key will be used) | [optional]
**pwd** | Option<**String**> |  | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**user_name** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


