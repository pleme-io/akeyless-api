# CreateClassicKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alg** | **String** | Classic Key type; options: [AES128GCM, AES256GCM, AES128SIV, AES256SIV, RSA1024, RSA2048, RSA3072, RSA4096, EC256, EC384, GPG] | 
**auto_rotate** | Option<**String**> | Whether to automatically rotate every rotation_interval days, or disable existing automatic rotation [true/false] | [optional]
**cert_file_data** | Option<**String**> | Certificate in a PEM format. | [optional]
**certificate_common_name** | Option<**String**> | Common name for the generated certificate. Relevant only for generate-self-signed-certificate. | [optional]
**certificate_country** | Option<**String**> | Country name for the generated certificate. Relevant only for generate-self-signed-certificate. | [optional]
**certificate_digest_algo** | Option<**String**> | Digest algorithm to be used for the certificate key signing. | [optional]
**certificate_format** | Option<**String**> |  | [optional]
**certificate_locality** | Option<**String**> | Locality for the generated certificate. Relevant only for generate-self-signed-certificate. | [optional]
**certificate_organization** | Option<**String**> | Organization name for the generated certificate. Relevant only for generate-self-signed-certificate. | [optional]
**certificate_province** | Option<**String**> | Province name for the generated certificate. Relevant only for generate-self-signed-certificate. | [optional]
**certificate_ttl** | Option<**i64**> | TTL in days for the generated certificate. Required only for generate-self-signed-certificate. | [optional]
**conf_file_data** | Option<**String**> | The csr config data in base64 encoding | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**expiration_event_in** | Option<**Vec<String>**> | How many days before the expiration of the certificate would you like to be notified. | [optional]
**generate_self_signed_certificate** | Option<**bool**> | Whether to generate a self signed certificate with the key. If set, --certificate-ttl must be provided. | [optional]
**gpg_alg** | Option<**String**> | gpg alg: Relevant only if GPG key type selected; options: [RSA1024, RSA2048, RSA3072, RSA4096, Ed25519] | [optional]
**hash_algorithm** | Option<**String**> | Specifies the hash algorithm used for the encryption key's operations, available options: [SHA256, SHA384, SHA512] | [optional][default to SHA256]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key_data** | Option<**String**> | Base64-encoded classic key value | [optional]
**metadata** | Option<**String**> | Deprecated - use description | [optional]
**name** | **String** | ClassicKey name | 
**protection_key_name** | Option<**String**> | The name of a key that used to encrypt the secret value (if empty, the account default protectionKey key will be used) | [optional]
**rotation_event_in** | Option<**Vec<String>**> | How many days before the rotation of the item would you like to be notified | [optional]
**rotation_interval** | Option<**String**> | The number of days to wait between every automatic rotation (1-365) | [optional]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


