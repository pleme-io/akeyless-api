# DecryptGpg

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ciphertext** | **String** | Ciphertext to be decrypted | 
**display_id** | Option<**String**> | The display id of the key to use in the decryption process | [optional]
**input_format** | Option<**String**> | Select default assumed format for the ciphertext. Currently supported options: [base64,raw] | [optional][default to base64]
**item_id** | Option<**i64**> | The item id of the key to use in the decryption process | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key_name** | **String** | The name of the key to use in the decryption process | 
**output_format** | Option<**String**> | If specified, the output will be formatted accordingly. options: [base64] | [optional]
**passphrase** | Option<**String**> | Passphrase that was used to generate the key | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


