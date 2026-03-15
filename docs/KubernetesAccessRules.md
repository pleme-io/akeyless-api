# KubernetesAccessRules

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alg** | Option<**String**> |  | [optional]
**audience** | Option<**String**> | Audience is an optional Kubernetes jwt claim to verify | [optional]
**bound_namespaces** | Option<**Vec<String>**> | A list of namespaces that the authentication is restricted to. | [optional]
**bound_pod_names** | Option<**Vec<String>**> | A list of pods names that the authentication is restricted to. | [optional]
**bound_service_account_names** | Option<**Vec<String>**> | A list of service account names that the authentication is restricted to. | [optional]
**gen_key_pair** | Option<**String**> | Generate public/private key (the private key is required for the K8S Auth Config in the Akeyless Gateway) | [optional]
**pub_key** | Option<**String**> | The public key value of the Kubernetes auth method configuration in the Akeyless Gateway. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


