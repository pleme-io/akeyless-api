# ImportPasswords

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accessibility** | Option<**String**> | for personal password manager | [optional][default to personal]
**format** | Option<**String**> | Password format type [LastPass/Chrome/Firefox/1password/keeper/bitwarden/dashlane] | [optional][default to LastPass]
**import_path** | **String** | File path | 
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**protection_key** | Option<**String**> | The name of a key that used to encrypt the secret value (if empty, the account default protectionKey key will be used) | [optional]
**target_folder** | Option<**String**> | Target folder for imported passwords | [optional][default to /]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**update_mode** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


