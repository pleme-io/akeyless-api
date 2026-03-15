# CertificateInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ext_key_usage** | Option<**Vec<i64>**> |  | [optional]
**key_usage** | Option<**i64**> | KeyUsage represents the set of actions that are valid for a given key. It's a bitmap of the KeyUsage* constants. | [optional]
**crl_distribution_points** | Option<**Vec<String>**> |  | [optional]
**dns_names** | Option<**Vec<String>**> |  | [optional]
**email_addresses** | Option<**Vec<String>**> |  | [optional]
**extensions** | Option<[**Vec<models::Extension>**](Extension.md)> |  | [optional]
**ip_addresses** | Option<**Vec<String>**> |  | [optional]
**is_ca** | Option<**bool**> |  | [optional]
**issuer** | Option<[**models::Name**](Name.md)> |  | [optional]
**issuing_certificate_url** | Option<**Vec<String>**> |  | [optional]
**key_size** | Option<**i64**> |  | [optional]
**not_after** | Option<**String**> |  | [optional]
**not_before** | Option<**String**> |  | [optional]
**ocsp_server** | Option<**Vec<String>**> |  | [optional]
**public_key_algorithm_name** | Option<**String**> |  | [optional]
**serial_number** | Option<**String**> |  | [optional]
**sha_1_fingerprint** | Option<**String**> |  | [optional]
**sha_256_fingerprint** | Option<**String**> |  | [optional]
**signature** | Option<**String**> |  | [optional]
**signature_algorithm_name** | Option<**String**> |  | [optional]
**subject** | Option<[**models::Name**](Name.md)> |  | [optional]
**subject_public_key** | Option<**String**> |  | [optional]
**uris** | Option<**Vec<String>**> |  | [optional]
**version** | Option<**i64**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


