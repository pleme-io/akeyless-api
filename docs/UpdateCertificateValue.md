# UpdateCertificateValue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**add_tag** | Option<**Vec<String>**> | List of the new tags that will be attached to this item | [optional]
**certificate_data** | Option<**String**> | Content of the certificate in a Base64 format. | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**expiration_event_in** | Option<**Vec<String>**> | How many days before the expiration of the certificate would you like to be notified. | [optional]
**format** | Option<**String**> | CertificateFormat of the certificate and private key, possible values: cer,crt,pem,pfx,p12. Required when passing inline certificate content with --certificate-data or --key-data, otherwise format is derived from the file extension. | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**keep_prev_version** | Option<**String**> | Whether to keep previous version [true/false]. If not set, use default according to account settings | [optional]
**key** | Option<**String**> | The name of a key to use to encrypt the certificate's key (if empty, the account default protectionKey key will be used) | [optional]
**key_data** | Option<**String**> | Content of the certificate's private key in a Base64 format. | [optional]
**metadata** | Option<**String**> | Deprecated - use description | [optional]
**name** | **String** | Certificate name | 
**rm_tag** | Option<**Vec<String>**> | List of the existent tags that will be removed from this item | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


