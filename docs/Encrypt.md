# Encrypt

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_id** | Option<**String**> | The display id of the key to use in the encryption process | [optional]
**encryption_context** | Option<**std::collections::HashMap<String, String>**> | name-value pair that specifies the encryption context to be used for authenticated encryption. If used here, the same value must be supplied to the decrypt command or decryption will fail | [optional]
**input_format** | Option<**String**> | Select default assumed format for any plaintext input. Currently supported options: [base64] | [optional]
**item_id** | Option<**i64**> | The item id of the key to use in the encryption process | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key_name** | **String** | The name of the key to use in the encryption process | 
**plaintext** | Option<**String**> | Data to be encrypted | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**version** | Option<**i32**> | key version (relevant only for classic key) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


