# DynamicSecretCreateK8s

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**custom_username_template** | Option<**String**> | Customize how temporary usernames are generated using go template | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**k8s_allowed_namespaces** | Option<**String**> | Comma-separated list of allowed K8S namespaces for the generated ServiceAccount (relevant only for k8s-service-account-type=dynamic) | [optional]
**k8s_cluster_ca_cert** | Option<**String**> | K8S cluster CA certificate | [optional]
**k8s_cluster_endpoint** | Option<**String**> | K8S cluster URL endpoint | [optional]
**k8s_cluster_name** | Option<**String**> | K8S cluster name | [optional]
**k8s_cluster_token** | Option<**String**> | K8S cluster Bearer token | [optional]
**k8s_namespace** | Option<**String**> | K8S Namespace where the ServiceAccount exists. | [optional]
**k8s_predefined_role_name** | Option<**String**> | The pre-existing Role or ClusterRole name to bind the generated ServiceAccount to (relevant only for k8s-service-account-type=dynamic) | [optional]
**k8s_predefined_role_type** | Option<**String**> | Specifies the type of the pre-existing K8S role [Role, ClusterRole] (relevant only for k8s-service-account-type=dynamic) | [optional]
**k8s_rolebinding_yaml_data** | Option<**String**> | Content of the yaml in a Base64 format. | [optional]
**k8s_rolebinding_yaml_def** | Option<**String**> | Path to yaml file that contains definitions of K8S role and role binding (relevant only for k8s-service-account-type=dynamic) | [optional]
**k8s_service_account** | Option<**String**> | K8S ServiceAccount to extract token from. | [optional]
**k8s_service_account_type** | Option<**String**> | K8S ServiceAccount type [fixed, dynamic]. | [optional]
**name** | **String** | Dynamic secret name | 
**producer_encryption_key_name** | Option<**String**> | Dynamic producer encryption key | [optional]
**secure_access_allow_port_forwading** | Option<**bool**> | Enable Port forwarding while using CLI access | [optional]
**secure_access_bastion_issuer** | Option<**String**> | Deprecated. use secure-access-certificate-issuer | [optional]
**secure_access_certificate_issuer** | Option<**String**> | Path to the SSH Certificate Issuer for your Akeyless Secure Access | [optional]
**secure_access_cluster_endpoint** | Option<**String**> | The K8s cluster endpoint URL | [optional]
**secure_access_dashboard_url** | Option<**String**> | The K8s dashboard url | [optional]
**secure_access_delay** | Option<**i64**> | The delay duration, in seconds, to wait after generating just-in-time credentials. Accepted range: 0-120 seconds | [optional]
**secure_access_enable** | Option<**String**> | Enable/Disable secure remote access [true/false] | [optional]
**secure_access_web** | Option<**bool**> | Enable Web Secure Remote Access | [optional][default to false]
**secure_access_web_browsing** | Option<**bool**> | Secure browser via Akeyless's Secure Remote Access (SRA) | [optional][default to false]
**secure_access_web_proxy** | Option<**bool**> | Web-Proxy via Akeyless's Secure Remote Access (SRA) | [optional][default to false]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**target_name** | Option<**String**> | Target name | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**use_gw_service_account** | Option<**bool**> | Use the GW's service account | [optional]
**user_ttl** | Option<**String**> | User TTL | [optional][default to 60m]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


