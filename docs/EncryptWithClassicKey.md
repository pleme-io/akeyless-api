# EncryptWithClassicKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_id** | **String** | The name of the key to use in the encryption process | 
**ignore_cache** | Option<**String**> | Retrieve the Secret value without checking the Gateway's cache [true/false]. This flag is only relevant when using the RestAPI | [optional][default to false]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**plaintext** | **String** | Data to be encrypted | 
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**version** | **i32** | classic key version | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


