# Hmac

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_id** | Option<**String**> | The display id of the key to use in the encryption process | [optional]
**hash_function** | Option<**String**> | Hash function [sha-256,sha-512] | [optional][default to sha-256]
**input_format** | Option<**String**> | Select default assumed format for any plaintext input. Currently supported options: [base64] | [optional]
**item_id** | Option<**i64**> | The item id of the key to use in the encryption process | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key_name** | **String** | The name of the key to use in the encryption process | 
**plaintext** | Option<**String**> | Data to perform hmac on | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


