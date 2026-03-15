# VerifyDataWithClassicKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | **String** | Data | 
**display_id** | Option<**String**> | The display id of the key to use in the verification process | [optional]
**hashed** | Option<**bool**> | Defines whether the data should be hashed as part of the signing. If true, the data will not be hashed | [optional][default to false]
**hashing_method** | Option<**String**> | HashingMethod | [optional][default to SHA256]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | The name of the key to use in the verification process | 
**signature** | **String** | The data's signature in a Base64 format. | 
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**version** | **i32** | classic key version | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


