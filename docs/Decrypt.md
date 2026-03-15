# Decrypt

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ciphertext** | Option<**String**> | Ciphertext to be decrypted in base64 encoded format | [optional]
**display_id** | Option<**String**> | The display id of the key to use in the decryption process | [optional]
**encryption_context** | Option<**std::collections::HashMap<String, String>**> | The encryption context. If this was specified in the encrypt command, it must be specified here or the decryption operation will fail | [optional]
**item_id** | Option<**i64**> | The item id of the key to use in the decryption process | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key_name** | **String** | The name of the key to use in the decryption process | 
**output_format** | Option<**String**> | If specified, the output will be formatted accordingly. options: [base64] | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**version** | Option<**i32**> | key version (relevant only for classic key) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


