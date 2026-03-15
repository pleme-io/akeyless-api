# GetRotatedSecretValue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**host** | Option<**String**> | Get rotated secret value of specific Host (relevant only for Linked Target) | [optional]
**ignore_cache** | Option<**String**> | Retrieve the Secret value without checking the Gateway's cache [true/false]. This flag is only relevant when using the RestAPI | [optional][default to false]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**names** | **String** | Secret name | 
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**version** | Option<**i32**> | Secret version | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


