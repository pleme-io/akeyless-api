# Configure

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_id** | Option<**String**> | Access ID | [optional]
**access_key** | Option<**String**> | Access Key | [optional]
**access_type** | Option<**String**> | Access Type (access_key/password/azure_ad/saml/oidc/aws_iam/gcp/k8s/cert) | [optional][default to access_key]
**account_id** | Option<**String**> | Account id (relevant only for access-type=password where the email address is associated with more than one account) | [optional]
**admin_email** | Option<**String**> | Email (relevant only for access-type=password) | [optional]
**admin_password** | Option<**String**> | Password (relevant only for access-type=password) | [optional]
**azure_ad_object_id** | Option<**String**> | Azure Active Directory ObjectId (relevant only for access-type=azure_ad) | [optional]
**azure_cloud** | Option<**String**> | Azure cloud environment to use. Values: AzureCloud (default), AzureUSGovernment, AzureChinaCloud. | [optional][default to AzureCloud]
**cert_data** | Option<**String**> | Certificate data encoded in base64. Used if file was not provided. (relevant only for access-type=cert in Curl Context) | [optional]
**cert_issuer_name** | Option<**String**> | Certificate Issuer Name | [optional]
**cert_username** | Option<**String**> | The username to sign in the SSH certificate (use a comma-separated list for more than one username) | [optional]
**default_location_prefix** | Option<**String**> | Default path prefix for name of items, targets and auth methods | [optional]
**disable_pafxfast** | Option<**String**> | Disable the FAST negotiation in the Kerberos authentication method | [optional]
**gateway_spn** | Option<**String**> | The service principal name of the gateway as registered in LDAP (i.e., HTTP/gateway) | [optional]
**gcp_audience** | Option<**String**> | GCP JWT audience | [optional][default to akeyless.io]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**k8s_auth_config_name** | Option<**String**> | The K8S Auth config name (relevant only for access-type=k8s) | [optional]
**kerberos_token** | Option<**String**> | KerberosToken represents a Kerberos token generated for the gateway SPN (Service Principal Name). | [optional]
**kerberos_username** | Option<**String**> | TThe username for the entry within the keytab to authenticate via Kerberos | [optional]
**key_data** | Option<**String**> | Private key data encoded in base64. Used if file was not provided.(relevant only for access-type=cert in Curl Context) | [optional]
**keytab_data** | Option<**String**> | Base64-encoded content of a valid keytab file, containing the service account's entry. | [optional]
**krb5_conf_data** | Option<**String**> | Base64-encoded content of a valid krb5.conf file, specifying the settings and parameters required for Kerberos authentication. | [optional]
**legacy_signing_alg_name** | Option<**bool**> | Set this option to output legacy ('ssh-rsa-cert-v01@openssh.com') signing algorithm name in the certificate. | [optional]
**oci_auth_type** | Option<**String**> | The type of the OCI configuration to use [instance/apikey/resource] (relevant only for access-type=oci) | [optional][default to apikey]
**oci_group_ocid** | Option<**Vec<String>**> | A list of Oracle Cloud IDs groups (relevant only for access-type=oci) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


