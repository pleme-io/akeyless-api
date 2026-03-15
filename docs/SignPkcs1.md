# SignPkcs1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_id** | Option<**String**> | The display id of the key to use in the signing process | [optional]
**hash_function** | Option<**String**> | HashFunction defines the hash function (e.g. sha-256) | [optional]
**input_format** | Option<**String**> | Select default assumed format for the plaintext message. Currently supported options: [base64] | [optional]
**item_id** | Option<**i64**> | The item id of the key to use in the signing process | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key_name** | Option<**String**> | The name of the RSA key to use in the signing process | [optional]
**message** | **String** | The message to be signed | 
**prehashed** | Option<**bool**> | Markes that the message is already hashed | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**version** | Option<**i32**> | The version of the key to use for signing | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


