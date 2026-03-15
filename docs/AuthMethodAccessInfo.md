# AuthMethodAccessInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_expires** | Option<**i64**> |  | [optional]
**access_id_alias** | Option<**String**> | for accounts where AccessId holds encrypted email this field will hold generated AccessId, for accounts based on regular AccessId it will be equal to accessId itself | [optional]
**allowed_client_type** | Option<**Vec<String>**> |  | [optional]
**api_key_access_rules** | Option<[**models::ApiKeyAccessRules**](APIKeyAccessRules.md)> |  | [optional]
**audit_logs_claims** | Option<**Vec<String>**> |  | [optional]
**aws_iam_access_rules** | Option<[**models::AwsiamAccessRules**](AWSIAMAccessRules.md)> |  | [optional]
**azure_ad_access_rules** | Option<[**models::AzureAdAccessRules**](AzureADAccessRules.md)> |  | [optional]
**cert_access_rules** | Option<[**models::CertAccessRules**](CertAccessRules.md)> |  | [optional]
**cidr_whitelist** | Option<**String**> |  | [optional]
**email_pass_access_rules** | Option<[**models::EmailPassAccessRules**](EmailPassAccessRules.md)> |  | [optional]
**force_sub_claims** | Option<**bool**> | if true the role associated with this auth method must include sub claims | [optional]
**gcp_access_rules** | Option<[**models::GcpAccessRules**](GCPAccessRules.md)> |  | [optional]
**gw_cidr_whitelist** | Option<**String**> |  | [optional]
**huawei_access_rules** | Option<[**models::HuaweiAccessRules**](HuaweiAccessRules.md)> |  | [optional]
**jwt_ttl** | Option<**i64**> |  | [optional]
**k8s_access_rules** | Option<[**models::KubernetesAccessRules**](KubernetesAccessRules.md)> |  | [optional]
**kerberos_access_rules** | Option<[**models::KerberosAccessRules**](KerberosAccessRules.md)> |  | [optional]
**ldap_access_rules** | Option<[**models::LdapAccessRules**](LDAPAccessRules.md)> |  | [optional]
**oauth2_access_rules** | Option<[**models::OAuth2AccessRules**](OAuth2AccessRules.md)> |  | [optional]
**oci_access_rules** | Option<[**models::OciAccessRules**](OCIAccessRules.md)> |  | [optional]
**oidc_access_rules** | Option<[**models::OidcAccessRules**](OIDCAccessRules.md)> |  | [optional]
**product_types** | Option<**Vec<String>**> | List of product types this auth method will be in use of | [optional]
**rules_type** | Option<**String**> |  | [optional]
**saml_access_rules** | Option<[**models::SamlAccessRules**](SAMLAccessRules.md)> |  | [optional]
**sub_claims_delimiters** | Option<**Vec<String>**> |  | [optional]
**universal_identity_access_rules** | Option<[**models::UniversalIdentityAccessRules**](UniversalIdentityAccessRules.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


