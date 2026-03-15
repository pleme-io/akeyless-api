# Auth

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_id** | Option<**String**> | Access ID | [optional]
**access_key** | Option<**String**> | Access key (relevant only for access-type=access_key) | [optional]
**access_type** | Option<**String**> | Access Type (access_key/password/saml/ldap/k8s/azure_ad/oidc/aws_iam/universal_identity/jwt/gcp/cert/oci/kerberos) | [optional][default to access_key]
**account_id** | Option<**String**> | Account id (relevant only for access-type=password where the email address is associated with more than one account) | [optional]
**admin_email** | Option<**String**> | Email (relevant only for access-type=password) | [optional]
**admin_password** | Option<**String**> | Password (relevant only for access-type=password) | [optional]
**azure_cloud** | Option<**String**> | Azure cloud environment to use. Values: AzureCloud (default), AzureUSGovernment, AzureChinaCloud. | [optional][default to AzureCloud]
**cert_challenge** | Option<**String**> | Certificate challenge encoded in base64. (relevant only for access-type=cert) | [optional]
**cert_data** | Option<**String**> | Certificate data encoded in base64. Used if file was not provided. (relevant only for access-type=cert) | [optional]
**cloud_id** | Option<**String**> | The cloud identity (relevant only for access-type=azure_ad,aws_iam,gcp) | [optional]
**debug** | Option<**bool**> |  | [optional]
**disable_pafxfast** | Option<**String**> | Disable the FAST negotiation in the Kerberos authentication method | [optional]
**gateway_spn** | Option<**String**> | The service principal name of the gateway as registered in LDAP (i.e., HTTP/gateway) | [optional]
**gateway_url** | Option<**String**> | Gateway URL relevant only for access-type=k8s/oauth2/saml/oidc | [optional]
**gcp_audience** | Option<**String**> | GCP JWT audience | [optional][default to akeyless.io]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**jwt** | Option<**String**> | The Json Web Token (relevant only for access-type=jwt/oidc) | [optional]
**k8s_auth_config_name** | Option<**String**> | The K8S Auth config name (relevant only for access-type=k8s) | [optional]
**k8s_service_account_token** | Option<**String**> | The K8S service account token. (relevant only for access-type=k8s) | [optional]
**kerberos_token** | Option<**String**> | KerberosToken represents a Kerberos token generated for the gateway SPN (Service Principal Name). | [optional]
**kerberos_username** | Option<**String**> | TThe username for the entry within the keytab to authenticate via Kerberos | [optional]
**key_data** | Option<**String**> | Private key data encoded in base64. Used if file was not provided.(relevant only for access-type=cert) | [optional]
**keytab_data** | Option<**String**> | Base64-encoded content of a valid keytab file, containing the service account's entry. | [optional]
**krb5_conf_data** | Option<**String**> | Base64-encoded content of a valid krb5.conf file, specifying the settings and parameters required for Kerberos authentication. | [optional]
**ldap_password** | Option<**String**> | LDAP password (relevant only for access-type=ldap) | [optional]
**oci_auth_type** | Option<**String**> | The type of the OCI configuration to use [instance/apikey/resource] (relevant only for access-type=oci) | [optional][default to apikey]
**oci_group_ocid** | Option<**Vec<String>**> | A list of Oracle Cloud IDs groups (relevant only for access-type=oci) | [optional]
**otp** | Option<**String**> |  | [optional]
**signed_cert_challenge** | Option<**String**> | Signed certificate challenge encoded in base64. (relevant only for access-type=cert) | [optional]
**uid_token** | Option<**String**> | The universal_identity token (relevant only for access-type=universal_identity) | [optional]
**use_remote_browser** | Option<**bool**> | Returns a link to complete the authentication remotely (relevant only for access-type=saml/oidc) | [optional]
**username** | Option<**String**> | LDAP username (relevant only for access-type=ldap) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


