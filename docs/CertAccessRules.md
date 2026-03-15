# CertAccessRules

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allowed_cors** | Option<**Vec<String>**> | a list of allowed cors domains if used for browser authentication | [optional]
**bound_common_names** | Option<**Vec<String>**> | A list of names. At least one must exist in the Common Name. Supports globbing. | [optional]
**bound_dns_sans** | Option<**Vec<String>**> | A list of DNS names. At least one must exist in the SANs. Supports globbing. | [optional]
**bound_email_sans** | Option<**Vec<String>**> | A list of Email Addresses. At least one must exist in the SANs. Supports globbing. | [optional]
**bound_extensions** | Option<**Vec<String>**> | A list of extensions formatted as \"oid:value\". Expects the extension value to be some type of ASN1 encoded string. All values must match. Supports globbing on \"value\". | [optional]
**bound_organizational_units** | Option<**Vec<String>**> | A list of Organizational Units names. At least one must exist in the OU field. | [optional]
**bound_uri_sans** | Option<**Vec<String>**> | A list of URIs. At least one must exist in the SANs. Supports globbing. | [optional]
**certificate** | Option<**String**> | Base64 encdoed PEM certificate | [optional]
**require_crl_dp** | Option<**bool**> | RequireCrlDp indicates whether CRL distribution points are required on the leaf client certificate, and whether CRL validation must be enforced during authentication. | [optional]
**revoked_cert_ids** | Option<**Vec<String>**> | A list of revoked cert ids | [optional]
**unique_identifier** | Option<**String**> | A unique identifier to distinguish different users | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


