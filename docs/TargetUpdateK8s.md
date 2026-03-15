# TargetUpdateK8s

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Description of the object | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**k8s_auth_type** | Option<**String**> | K8S auth type [token/certificate] | [optional][default to token]
**k8s_client_certificate** | Option<**String**> | Content of the k8 client certificate (PEM format) in a Base64 format | [optional]
**k8s_client_key** | Option<**String**> | Content of the k8 client private key (PEM format) in a Base64 format | [optional]
**k8s_cluster_ca_cert** | Option<**String**> | K8S cluster CA certificate | [optional]
**k8s_cluster_endpoint** | Option<**String**> | K8S cluster URL endpoint | [optional]
**k8s_cluster_name** | Option<**String**> | K8S cluster name | [optional]
**k8s_cluster_token** | Option<**String**> | K8S cluster Bearer token | [optional]
**keep_prev_version** | Option<**String**> | Whether to keep previous version [true/false]. If not set, use default according to account settings | [optional]
**key** | Option<**String**> | The name of a key that used to encrypt the target secret value (if empty, the account default protectionKey key will be used) | [optional]
**max_versions** | Option<**String**> | Set the maximum number of versions, limited by the account settings defaults. | [optional]
**name** | **String** | Target name | 
**new_name** | Option<**String**> | New target name | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**use_gw_service_account** | Option<**bool**> | Use the GW's service account | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


