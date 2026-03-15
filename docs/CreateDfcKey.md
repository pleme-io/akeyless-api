# CreateDfcKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alg** | **String** | DFCKey type; options: [AES128GCM, AES256GCM, AES128SIV, AES256SIV, AES128CBC, AES256CBC, RSA1024, RSA2048, RSA3072, RSA4096] | 
**auto_rotate** | Option<**String**> | Whether to automatically rotate every rotation_interval days, or disable existing automatic rotation [true/false] | [optional]
**certificate_common_name** | Option<**String**> | Common name for the generated certificate. Relevant only for generate-self-signed-certificate. | [optional]
**certificate_country** | Option<**String**> | Country name for the generated certificate. Relevant only for generate-self-signed-certificate. | [optional]
**certificate_digest_algo** | Option<**String**> | Digest algorithm to be used for the certificate key signing. | [optional]
**certificate_format** | Option<**String**> |  | [optional]
**certificate_locality** | Option<**String**> | Locality for the generated certificate. Relevant only for generate-self-signed-certificate. | [optional]
**certificate_organization** | Option<**String**> | Organization name for the generated certificate. Relevant only for generate-self-signed-certificate. | [optional]
**certificate_province** | Option<**String**> | Province name for the generated certificate. Relevant only for generate-self-signed-certificate. | [optional]
**certificate_ttl** | Option<**i64**> | TTL in days for the generated certificate. Required only for generate-self-signed-certificate. | [optional]
**conf_file_data** | Option<**String**> | The csr config data in base64 encoding | [optional]
**customer_frg_id** | Option<**String**> | The customer fragment ID that will be used to create the DFC key (if empty, the key will be created independently of a customer fragment) | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**expiration_event_in** | Option<**Vec<String>**> | How many days before the expiration of the certificate would you like to be notified. | [optional]
**generate_self_signed_certificate** | Option<**bool**> | Whether to generate a self signed certificate with the key. If set, --certificate-ttl must be provided. | [optional]
**hash_algorithm** | Option<**String**> | Specifies the hash algorithm used for the encryption key's operations, available options: [SHA256, SHA384, SHA512] | [optional][default to SHA256]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**metadata** | Option<**String**> | Deprecated - use description | [optional]
**name** | **String** | DFCKey name | 
**rotation_event_in** | Option<**Vec<String>**> | How many days before the rotation of the item would you like to be notified | [optional]
**rotation_interval** | Option<**String**> | The number of days to wait between every automatic rotation (7-365) | [optional]
**split_level** | Option<**i64**> | The number of fragments that the item will be split into (not includes customer fragment) | [optional][default to 3]
**tag** | Option<**Vec<String>**> | List of the tags attached to this DFC key | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


