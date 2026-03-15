# EncryptFile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_id** | Option<**String**> | The display id of the key to use in the encryption process | [optional]
**r#in** | **String** | Path to the file to be encrypted. If not provided, the content will be taken from stdin | 
**item_id** | Option<**i64**> | The item id of the key to use in the encryption process | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key_name** | **String** | The name of the key to use in the encryption process | 
**out** | Option<**String**> | Path to the output file. If not provided, the output will be sent to stdout | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


