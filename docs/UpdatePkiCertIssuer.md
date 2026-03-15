# UpdatePkiCertIssuer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**add_tag** | Option<**Vec<String>**> | List of the new tags that will be attached to this item | [optional]
**allow_any_name** | Option<**bool**> | If set, clients can request certificates for any CN | [optional]
**allow_copy_ext_from_csr** | Option<**bool**> | If set, will allow copying the extra extensions from the csr file (if given) | [optional]
**allow_subdomains** | Option<**bool**> | If set, clients can request certificates for subdomains of the allowed domains | [optional]
**allowed_domains** | Option<**String**> | A list of the allowed domains that clients can request to be included in the certificate (in a comma-delimited list) | [optional]
**allowed_extra_extensions** | Option<**String**> | A json string containing the allowed extra extensions for the pki cert issuer | [optional]
**allowed_ip_sans** | Option<**String**> | A list of the allowed CIDRs for ips that clients can request to be included in the certificate as part of the IP Subject Alternative Names (in a comma-delimited list) | [optional]
**allowed_uri_sans** | Option<**String**> | A list of the allowed URIs that clients can request to be included in the certificate as part of the URI Subject Alternative Names (in a comma-delimited list) | [optional]
**auto_renew** | Option<**bool**> | Automatically renew certificates before expiration | [optional]
**client_flag** | Option<**bool**> | If set, certificates will be flagged for client auth use | [optional]
**code_signing_flag** | Option<**bool**> | If set, certificates will be flagged for code signing use | [optional]
**country** | Option<**String**> | A comma-separated list of countries that will be set in the issued certificate | [optional]
**create_private_crl** | Option<**bool**> | Set this to allow the issuer will expose a CRL endpoint in the Gateway | [optional]
**create_private_ocsp** | Option<**bool**> | Set this to enable an OCSP endpoint in the Gateway and include its URL in AIA | [optional]
**create_public_crl** | Option<**bool**> | Set this to allow the cert issuer will expose a public CRL endpoint | [optional]
**create_public_ocsp** | Option<**bool**> | Set this to enable a public OCSP endpoint and include its URL in AIA (served by UAM and includes account id) | [optional]
**critical_key_usage** | Option<**String**> | Mark key usage as critical [true/false] | [optional][default to true]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**destination_path** | Option<**String**> | A path in which to save generated certificates | [optional]
**disable_wildcards** | Option<**bool**> | If set, generation of wildcard certificates will be disabled. | [optional]
**enable_acme** | Option<**bool**> | If set, the cert issuer will support the acme protocol | [optional]
**expiration_event_in** | Option<**Vec<String>**> | How many days before the expiration of the certificate would you like to be notified. | [optional]
**gw_cluster_url** | Option<**String**> | The GW cluster URL to issue the certificate from. Required in Public CA mode, to allow CRLs on private CA, or to enable ACME | [optional]
**is_ca** | Option<**bool**> | If set, the basic constraints extension will be added to certificate | [optional]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key_usage** | Option<**String**> | key-usage | [optional][default to DigitalSignature,KeyAgreement,KeyEncipherment]
**locality** | Option<**String**> | A comma-separated list of localities that will be set in the issued certificate | [optional]
**max_path_len** | Option<**i64**> | The maximum path length for the generated certificate. -1, means unlimited | [optional][default to -1]
**metadata** | Option<**String**> | Deprecated - use description | [optional]
**name** | **String** | PKI certificate issuer name | 
**new_name** | Option<**String**> | New item name | [optional]
**not_enforce_hostnames** | Option<**bool**> | If set, any names are allowed for CN and SANs in the certificate and not only a valid host name | [optional]
**not_require_cn** | Option<**bool**> | If set, clients can request certificates without a CN | [optional]
**ocsp_ttl** | Option<**String**> | OCSP NextUpdate window for OCSP responses (min 10m). Supports s,m,h,d suffix. | [optional]
**organizational_units** | Option<**String**> | A comma-separated list of organizational units (OU) that will be set in the issued certificate | [optional]
**organizations** | Option<**String**> | A comma-separated list of organizations (O) that will be set in the issued certificate | [optional]
**postal_code** | Option<**String**> | A comma-separated list of postal codes that will be set in the issued certificate | [optional]
**protect_certificates** | Option<**bool**> | Whether to protect generated certificates from deletion | [optional]
**province** | Option<**String**> | A comma-separated list of provinces that will be set in the issued certificate | [optional]
**rm_tag** | Option<**Vec<String>**> | List of the existent tags that will be removed from this item | [optional]
**scheduled_renew** | Option<**i64**> | Number of days before expiration to renew certificates | [optional]
**server_flag** | Option<**bool**> | If set, certificates will be flagged for server auth use | [optional]
**signer_key_name** | Option<**String**> | A key to sign the certificate with, required in Private CA mode | [optional]
**street_address** | Option<**String**> | A comma-separated list of street addresses that will be set in the issued certificate | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**ttl** | **String** | The maximum requested Time To Live for issued certificates, in seconds. In case of Public CA, this is based on the CA target's supported maximum TTLs | 
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


