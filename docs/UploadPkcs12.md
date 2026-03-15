# UploadPkcs12

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**customer_frg_id** | Option<**String**> | The customer fragment ID that will be used to split the key (if empty, the key will be created independently of a customer fragment) | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**r#in** | **String** | PKCS#12 input file (private key and certificate only) | 
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**metadata** | Option<**String**> | Deprecated - use description | [optional]
**name** | **String** | Name of key to be created | 
**passphrase** | **String** | Passphrase to unlock the pkcs#12 bundle | 
**split_level** | Option<**i64**> | The number of fragments that the item will be split into | [optional][default to 3]
**tag** | Option<**Vec<String>**> | List of the tags attached to this key | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


