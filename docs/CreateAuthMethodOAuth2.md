# CreateAuthMethodOAuth2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_expires** | Option<**i64**> | Access expiration date in Unix timestamp (select 0 for access without expiry date) | [optional][default to 0]
**allowed_client_type** | Option<**Vec<String>**> | limit the auth method usage for specific client types [cli,ui,gateway-admin,sdk,mobile,extension] | [optional]
**audience** | Option<**String**> | The audience in the JWT | [optional]
**audit_logs_claims** | Option<**Vec<String>**> | Subclaims to include in audit logs, e.g \"--audit-logs-claims email --audit-logs-claims username\" | [optional]
**bound_client_ids** | Option<**Vec<String>**> | The clients ids that the access is restricted to | [optional]
**bound_ips** | Option<**Vec<String>**> | A CIDR whitelist with the IPs that the access is restricted to | [optional]
**cert** | Option<**String**> | CertificateFile Path to a file that contain the certificate in a PEM format. | [optional]
**cert_file_data** | Option<**String**> | CertificateFileData PEM Certificate in a Base64 format. | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Auth Method description | [optional]
**expiration_event_in** | Option<**Vec<String>**> | How many days before the expiration of the auth method would you like to be notified. | [optional]
**force_sub_claims** | Option<**bool**> | if true: enforce role-association must include sub claims | [optional]
**gateway_url** | Option<**String**> | Akeyless Gateway URL (Configuration Management port). Relevant only when the jwks-uri is accessible only from the gateway. | [optional]
**gw_bound_ips** | Option<**Vec<String>**> | A CIDR whitelist with the GW IPs that the access is restricted to | [optional]
**issuer** | Option<**String**> | Issuer URL | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**jwks_json_data** | Option<**String**> | The JSON Web Key Set (JWKS) that containing the public keys that should be used to verify any JSON Web Token (JWT) issued by the authorization server. base64 encoded string | [optional]
**jwks_uri** | Option<**String**> | The URL to the JSON Web Key Set (JWKS) that containing the public keys that should be used to verify any JSON Web Token (JWT) issued by the authorization server. | [optional]
**jwt_ttl** | Option<**i64**> | Jwt TTL | [optional][default to 0]
**name** | **String** | Auth Method name | 
**product_type** | Option<**Vec<String>**> | Choose the relevant product type for the auth method [sm, sra, pm, dp, ca] | [optional]
**subclaims_delimiters** | Option<**Vec<String>**> | A list of additional sub claims delimiters (relevant only for SAML, OIDC, OAuth2/JWT) | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**unique_identifier** | **String** | A unique identifier (ID) value should be configured for OAuth2, LDAP and SAML authentication method types and is usually a value such as the email, username, or upn for example. Whenever a user logs in with a token, these authentication types issue a \"sub claim\" that contains details uniquely identifying that user. This sub claim includes a key containing the ID value that you configured, and is used to distinguish between different users from within the same organization. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


