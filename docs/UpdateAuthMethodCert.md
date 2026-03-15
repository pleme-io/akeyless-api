# UpdateAuthMethodCert

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_expires** | Option<**i64**> | Access expiration date in Unix timestamp (select 0 for access without expiry date) | [optional][default to 0]
**allowed_client_type** | Option<**Vec<String>**> | limit the auth method usage for specific client types [cli,ui,gateway-admin,sdk,mobile,extension] | [optional]
**allowed_cors** | Option<**String**> | Comma separated list of allowed CORS domains to be validated as part of the authentication flow. | [optional]
**audit_logs_claims** | Option<**Vec<String>**> | Subclaims to include in audit logs, e.g \"--audit-logs-claims email --audit-logs-claims username\" | [optional]
**bound_common_names** | Option<**Vec<String>**> | A list of names. At least one must exist in the Common Name. Supports globbing. | [optional]
**bound_dns_sans** | Option<**Vec<String>**> | A list of DNS names. At least one must exist in the SANs. Supports globbing. | [optional]
**bound_email_sans** | Option<**Vec<String>**> | A list of Email Addresses. At least one must exist in the SANs. Supports globbing. | [optional]
**bound_extensions** | Option<**Vec<String>**> | A list of extensions formatted as \"oid:value\". Expects the extension value to be some type of ASN1 encoded string. All values much match. Supports globbing on \"value\". | [optional]
**bound_ips** | Option<**Vec<String>**> | A CIDR whitelist with the IPs that the access is restricted to | [optional]
**bound_organizational_units** | Option<**Vec<String>**> | A list of Organizational Units names. At least one must exist in the OU field. | [optional]
**bound_uri_sans** | Option<**Vec<String>**> | A list of URIs. At least one must exist in the SANs. Supports globbing. | [optional]
**certificate_data** | Option<**String**> | The certificate data in base64, if no file was provided | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Auth Method description | [optional]
**expiration_event_in** | Option<**Vec<String>**> | How many days before the expiration of the auth method would you like to be notified. | [optional]
**force_sub_claims** | Option<**bool**> | if true: enforce role-association must include sub claims | [optional]
**gw_bound_ips** | Option<**Vec<String>**> | A CIDR whitelist with the GW IPs that the access is restricted to | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**jwt_ttl** | Option<**i64**> | Jwt TTL | [optional][default to 0]
**name** | **String** | Auth Method name | 
**new_name** | Option<**String**> | Auth Method new name | [optional]
**product_type** | Option<**Vec<String>**> | Choose the relevant product type for the auth method [sm, sra, pm, dp, ca] | [optional]
**require_crl_dp** | Option<**bool**> | Require certificate CRL distribution points (CDP) and enforce CRL validation during authentication. | [optional]
**revoked_cert_ids** | Option<**Vec<String>**> | A list of revoked cert ids | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**unique_identifier** | **String** | A unique identifier (ID) value should be configured, such as common_name or organizational_unit Whenever a user logs in with a token, these authentication types issue a \"sub claim\" that contains details uniquely identifying that user. This sub claim includes a key containing the ID value that you configured, and is used to distinguish between different users from within the same organization. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


