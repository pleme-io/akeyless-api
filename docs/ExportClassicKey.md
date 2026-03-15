# ExportClassicKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accessibility** | Option<**String**> | for personal password manager | [optional][default to regular]
**export_public_key** | Option<**bool**> | Use this option to output only public key | [optional][default to false]
**ignore_cache** | Option<**String**> | Retrieve the Secret value without checking the Gateway's cache [true/false]. This flag is only relevant when using the RestAPI | [optional][default to false]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | ClassicKey name | 
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**version** | Option<**i32**> | Classic key version | [optional]
**wrapping_key_name** | Option<**String**> | Classic key name to wrap the key material with | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


