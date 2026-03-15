# PkiCertificateIssueDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acme_enabled** | Option<**bool**> |  | [optional]
**allow_any_name** | Option<**bool**> |  | [optional]
**allow_copy_ext_from_csr** | Option<**bool**> |  | [optional]
**allow_subdomains** | Option<**bool**> |  | [optional]
**allowed_domains_list** | Option<**Vec<String>**> |  | [optional]
**allowed_extra_extensions** | Option<[**std::collections::HashMap<String, Vec<String>>**](Vec.md)> |  | [optional]
**allowed_ip_sans** | Option<**Vec<String>**> |  | [optional]
**allowed_uri_sans** | Option<**Vec<String>**> |  | [optional]
**auto_renew_certificate** | Option<**bool**> |  | [optional]
**basic_constraints_valid_for_non_ca** | Option<**bool**> |  | [optional]
**certificate_authority_mode** | Option<**String**> |  | [optional]
**client_flag** | Option<**bool**> |  | [optional]
**code_signing_flag** | Option<**bool**> |  | [optional]
**country** | Option<**Vec<String>**> |  | [optional]
**create_private_crl** | Option<**bool**> |  | [optional]
**create_private_ocsp** | Option<**bool**> | CreatePrivateOcsp enables exposing an OCSP endpoint on the Gateway and embedding its URL in the AIA extension of issued certificates. | [optional]
**create_public_crl** | Option<**bool**> |  | [optional]
**create_public_ocsp** | Option<**bool**> | CreatePublicOcsp enables exposing a public OCSP endpoint on the Gateway and embedding its URL in the AIA extension of issued certificates. | [optional]
**destination_path** | Option<**String**> | DestinationPath is the destination to save generated certificates | [optional]
**disable_wildcards** | Option<**bool**> |  | [optional]
**enforce_hostnames** | Option<**bool**> |  | [optional]
**expiration_events** | Option<[**Vec<models::CertificateExpirationEvent>**](CertificateExpirationEvent.md)> | ExpirationNotification holds a list of expiration notices that should be sent in case a certificate is about to expire, this value is being propagated to the Certificate resources that are created | [optional]
**gw_cluster_id** | Option<**i64**> |  | [optional]
**gw_cluster_url** | Option<**String**> | GWClusterURL is required when CAMode is \"public\" and it defines the cluster URL the PKI should be issued from. The GW cluster must have permissions to read associated target's details | [optional]
**is_ca** | Option<**bool**> |  | [optional]
**key_bits** | Option<**i64**> |  | [optional]
**key_type** | Option<**String**> |  | [optional]
**key_usage_list** | Option<**Vec<String>**> |  | [optional]
**locality** | Option<**Vec<String>**> |  | [optional]
**max_path_len** | Option<**i64**> |  | [optional]
**non_critical_key_usage** | Option<**bool**> |  | [optional]
**not_before_duration** | Option<**i64**> | A Duration represents the elapsed time between two instants as an int64 nanosecond count. The representation limits the largest representable duration to approximately 290 years. | [optional]
**ocsp_next_update** | Option<**i64**> | OcspNextUpdate defines the desired NextUpdate window for OCSP responses. Value is in seconds; 0 means not set. Minimum enforced is 10 minutes. | [optional]
**organization_list** | Option<**Vec<String>**> |  | [optional]
**organization_unit_list** | Option<**Vec<String>**> |  | [optional]
**pki_issuer_type** | Option<**String**> |  | [optional]
**postal_code** | Option<**Vec<String>**> |  | [optional]
**protect_generated_certificates** | Option<**bool**> | ProtectGeneratedCertificates dictates whether the created certificates should be protected from deletion | [optional]
**province** | Option<**Vec<String>**> |  | [optional]
**renew_before_expiration_in_days** | Option<**i64**> |  | [optional]
**require_cn** | Option<**bool**> |  | [optional]
**server_flag** | Option<**bool**> |  | [optional]
**street_address** | Option<**Vec<String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


