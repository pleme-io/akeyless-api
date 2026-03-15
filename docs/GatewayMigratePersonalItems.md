# GatewayMigratePersonalItems

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**param_1password_email** | Option<**String**> | 1Password user email to connect to the API | [optional]
**param_1password_password** | Option<**String**> | 1Password user password to connect to the API | [optional]
**param_1password_secret_key** | Option<**String**> | 1Password user secret key to connect to the API | [optional]
**param_1password_url** | Option<**String**> | 1Password api container url | [optional]
**param_1password_vaults** | Option<**Vec<String>**> | 1Password list of vault to get the items from | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**protection_key** | Option<**String**> | The name of a key that used to encrypt the secret value | [optional]
**target_location** | Option<**String**> | Target location in your Akeyless personal folder for migrated secrets | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**r#type** | Option<**String**> | Migration type for now only 1password. | [optional][default to 1password]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


