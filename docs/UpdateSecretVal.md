# UpdateSecretVal

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accessibility** | Option<**String**> | for personal password manager | [optional][default to regular]
**custom_field** | Option<**std::collections::HashMap<String, String>**> | For Password Management use, additional fields | [optional]
**format** | Option<**String**> | Secret format [text/json/key-value] (relevant only for type 'generic') | [optional][default to text]
**inject_url** | Option<**Vec<String>**> | For Password Management use, reflect the website context | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**keep_prev_version** | Option<**String**> | Whether to keep previous version [true/false]. If not set, use default according to account settings | [optional]
**key** | Option<**String**> | The name of a key that used to encrypt the secret value (if empty, the account default protectionKey key will be used) | [optional]
**last_version** | Option<**i32**> | The last version number before the update | [optional]
**multiline** | Option<**bool**> | The provided value is a multiline value (separated by '\\n') | [optional]
**name** | **String** | Secret name | 
**new_version** | Option<**bool**> | Deprecated | [optional]
**password** | Option<**String**> | For Password Management use, additional fields | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**username** | Option<**String**> | For Password Management use | [optional]
**value** | **String** | The secret value (relevant only for type 'generic') | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


