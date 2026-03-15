# DynamicSecretUpdateGke

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**gke_account_key** | Option<**String**> | GKE Service Account key file path | [optional]
**gke_cluster_cert** | Option<**String**> | GKE cluster CA certificate | [optional]
**gke_cluster_endpoint** | Option<**String**> | GKE cluster URL endpoint | [optional]
**gke_cluster_name** | Option<**String**> | GKE cluster name | [optional]
**gke_service_account_email** | Option<**String**> | GKE service account email | [optional]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Dynamic secret name | 
**new_name** | Option<**String**> | Dynamic secret name | [optional]
**producer_encryption_key_name** | Option<**String**> | Dynamic producer encryption key | [optional]
**secure_access_allow_port_forwading** | Option<**bool**> | Enable Port forwarding while using CLI access | [optional]
**secure_access_bastion_issuer** | Option<**String**> | Deprecated. use secure-access-certificate-issuer | [optional]
**secure_access_certificate_issuer** | Option<**String**> | Path to the SSH Certificate Issuer for your Akeyless Secure Access | [optional]
**secure_access_cluster_endpoint** | Option<**String**> | The K8s cluster endpoint URL | [optional]
**secure_access_delay** | Option<**i64**> | The delay duration, in seconds, to wait after generating just-in-time credentials. Accepted range: 0-120 seconds | [optional]
**secure_access_enable** | Option<**String**> | Enable/Disable secure remote access [true/false] | [optional]
**secure_access_web** | Option<**bool**> | Enable Web Secure Remote Access | [optional][default to false]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**target_name** | Option<**String**> | Target name | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**user_ttl** | Option<**String**> | User TTL | [optional][default to 60m]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


