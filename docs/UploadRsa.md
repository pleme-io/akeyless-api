# UploadRsa

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alg** | **String** | Key type. options: [RSA1024, RSA2048, RSA3072, RSA4096] | 
**cert_file_data** | Option<**String**> | Certificate in a PEM format. | [optional]
**certificate_format** | Option<**String**> |  | [optional]
**customer_frg_id** | Option<**String**> | The customer fragment ID that will be used to split the key (if empty, the key will be created independently of a customer fragment) | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**expiration_event_in** | Option<**Vec<String>**> | How many days before the expiration of the certificate would you like to be notified. | [optional]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**metadata** | Option<**String**> | Deprecated - use description | [optional]
**name** | **String** | Name of key to be created | 
**overwrite** | Option<**String**> | When the overwrite flag is set, this command will only update an existing key [true/false] | [optional][default to false]
**rsa_file_data** | Option<**String**> | RSA private key data, base64 encoded | [optional]
**split_level** | Option<**i64**> | The number of fragments that the item will be split into | [optional][default to 3]
**tag** | Option<**Vec<String>**> | List of the tags attached to this key | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


