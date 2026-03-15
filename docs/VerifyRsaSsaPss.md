# VerifyRsaSsaPss

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_id** | Option<**String**> | The display id of the RSA key to use in the verification process | [optional]
**hash_function** | Option<**String**> | HashFunction defines the hash function (e.g. sha-256) | [optional]
**item_id** | Option<**i64**> | The item id of the RSA key to use in the verification process | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key_name** | Option<**String**> | The name of the RSA key to use in the verification process | [optional]
**message** | **String** | The input message to verify in a base64 format | 
**prehashed** | Option<**bool**> | Markes that the message is already hashed | [optional]
**signature** | **String** | The message's signature | 
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**version** | Option<**i32**> | The version of the key to use for verification | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


