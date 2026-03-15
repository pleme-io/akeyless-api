# PolicyUpdateKeys

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allowed_algorithms** | Option<**Vec<String>**> | Specify allowed key algorithms (e.g., [RSA2048,AES128GCM]) | [optional]
**allowed_key_names** | Option<**Vec<String>**> | Specify allowed protection key names. To enforce using the account's default protection key, use 'default-account-key' | [optional]
**allowed_key_types** | Option<**Vec<String>**> | Specify allowed key protection types (dfc, classic-key) | [optional]
**id** | **String** | Policy id | 
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**max_rotation_interval_days** | Option<**i32**> | Set the maximum rotation interval for automatic key rotation. | [optional]
**object_types** | Option<**Vec<String>**> | The object type this policy will apply to (items, targets) | [optional]
**path** | Option<**String**> | The path the policy refers to | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


