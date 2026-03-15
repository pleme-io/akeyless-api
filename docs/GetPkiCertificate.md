# GetPkiCertificate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alt_names** | Option<**String**> | The Subject Alternative Names to be included in the PKI certificate (in a comma-separated list) (if CSR is supplied this flag is ignored and any DNS.* names are taken from it) | [optional]
**cert_issuer_name** | **String** | The name of the PKI certificate issuer | 
**common_name** | Option<**String**> | The common name to be included in the PKI certificate (if CSR is supplied this flag is ignored and the CSR subject CN is taken) | [optional]
**csr_data_base64** | Option<**String**> | Certificate Signing Request contents encoded in base64 to generate the certificate with | [optional]
**extended_key_usage** | Option<**String**> | A comma-separated list of extended key usage requests which will be used for certificate issuance. Supported values: 'clientauth', 'serverauth', 'codesigning'. If critical is present the extension will be marked as critical | [optional]
**extra_extensions** | Option<**String**> | A json string that defines the requested extra extensions for the certificate | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key_data_base64** | Option<**String**> | PKI key file contents. If this option is used, the certificate will be printed to stdout | [optional]
**max_path_len** | Option<**i64**> | The maximum path length for the generated certificate. -1, means unlimited unless the signing certificate has a maximum path length set | [optional][default to -1]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**ttl** | Option<**String**> | Updated certificate lifetime in seconds (must be less than the Certificate Issuer default TTL) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**uri_sans** | Option<**String**> | The URI Subject Alternative Names to be included in the PKI certificate (in a comma-separated list) (if CSR is supplied this flag is ignored and any URI.* names are taken from it) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


