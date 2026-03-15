# CreateAuthMethodOidc

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_expires** | Option<**i64**> | Access expiration date in Unix timestamp (select 0 for access without expiry date) | [optional][default to 0]
**allowed_client_type** | Option<**Vec<String>**> | limit the auth method usage for specific client types [cli,ui,gateway-admin,sdk,mobile,extension] | [optional]
**allowed_redirect_uri** | Option<**Vec<String>**> | Allowed redirect URIs after the authentication | [optional]
**audience** | Option<**String**> | Audience claim to be used as part of the authentication flow. In case set, it must match the one configured on the Identity Provider's Application | [optional]
**audit_logs_claims** | Option<**Vec<String>**> | Subclaims to include in audit logs, e.g \"--audit-logs-claims email --audit-logs-claims username\" | [optional]
**bound_ips** | Option<**Vec<String>**> | A CIDR whitelist with the IPs that the access is restricted to | [optional]
**client_id** | Option<**String**> | Client ID | [optional]
**client_secret** | Option<**String**> | Client Secret | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Auth Method description | [optional]
**expiration_event_in** | Option<**Vec<String>**> | How many days before the expiration of the auth method would you like to be notified. | [optional]
**force_sub_claims** | Option<**bool**> | if true: enforce role-association must include sub claims | [optional]
**gw_bound_ips** | Option<**Vec<String>**> | A CIDR whitelist with the GW IPs that the access is restricted to | [optional]
**issuer** | Option<**String**> | Issuer URL | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**jwt_ttl** | Option<**i64**> | Jwt TTL | [optional][default to 0]
**name** | **String** | Auth Method name | 
**product_type** | Option<**Vec<String>**> | Choose the relevant product type for the auth method [sm, sra, pm, dp, ca] | [optional]
**required_scopes** | Option<**Vec<String>**> | RequiredScopes is a list of required scopes that the oidc method will request from the oidc provider and the user must approve | [optional]
**required_scopes_prefix** | Option<**String**> | RequiredScopesPrefix is a a prefix to add to all required-scopes when requesting them from the oidc server (for example, azures' Application ID URI) | [optional]
**subclaims_delimiters** | Option<**Vec<String>**> | A list of additional sub claims delimiters (relevant only for SAML, OIDC, OAuth2/JWT) | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**unique_identifier** | **String** | A unique identifier (ID) value should be configured for OIDC, OAuth2, LDAP and SAML authentication method types and is usually a value such as the email, username, or upn for example. Whenever a user logs in with a token, these authentication types issue a \"sub claim\" that contains details uniquely identifying that user. This sub claim includes a key containing the ID value that you configured, and is used to distinguish between different users from within the same organization. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


