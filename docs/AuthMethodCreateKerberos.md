# AuthMethodCreateKerberos

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_expires** | Option<**i64**> | Access expiration date in Unix timestamp (select 0 for access without expiry date) | [optional][default to 0]
**allowed_client_type** | Option<**Vec<String>**> | limit the auth method usage for specific client types [cli,ui,gateway-admin,sdk,mobile,extension] | [optional]
**audit_logs_claims** | Option<**Vec<String>**> | Subclaims to include in audit logs, e.g \"--audit-logs-claims email --audit-logs-claims username\" | [optional]
**bind_dn** | Option<**String**> |  | [optional]
**bind_dn_password** | Option<**String**> |  | [optional]
**bound_ips** | Option<**Vec<String>**> | A CIDR whitelist with the IPs that the access is restricted to | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Auth Method description | [optional]
**expiration_event_in** | Option<**Vec<String>**> | How many days before the expiration of the auth method would you like to be notified. | [optional]
**force_sub_claims** | Option<**bool**> | if true: enforce role-association must include sub claims | [optional]
**group_attr** | Option<**String**> |  | [optional]
**group_dn** | Option<**String**> |  | [optional]
**group_filter** | Option<**String**> |  | [optional]
**gw_bound_ips** | Option<**Vec<String>**> | A CIDR whitelist with the GW IPs that the access is restricted to | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**jwt_ttl** | Option<**i64**> | Jwt TTL | [optional][default to 0]
**keytab_file_data** | Option<**String**> |  | [optional]
**keytab_file_path** | Option<**String**> |  | [optional]
**krb5_conf_data** | Option<**String**> |  | [optional]
**krb5_conf_path** | Option<**String**> |  | [optional]
**ldap_anonymous_search** | Option<**bool**> |  | [optional]
**ldap_ca_cert** | Option<**String**> |  | [optional]
**ldap_url** | Option<**String**> |  | [optional]
**name** | **String** | Auth Method name | 
**product_type** | Option<**Vec<String>**> | Choose the relevant product type for the auth method [sm, sra, pm, dp, ca] | [optional]
**subclaims_delimiters** | Option<**Vec<String>**> | A list of additional sub claims delimiters (relevant only for SAML, OIDC, OAuth2/JWT) | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**unique_identifier** | Option<**String**> | A unique identifier (ID) value which is a \"sub claim\" name that contains details uniquely identifying that resource. This \"sub claim\" is used to distinguish between different identities. | [optional]
**user_attribute** | Option<**String**> |  | [optional]
**user_dn** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


