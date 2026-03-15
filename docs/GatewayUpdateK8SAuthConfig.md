# GatewayUpdateK8SAuthConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_id** | **String** | The access ID of the Kubernetes auth method | 
**cluster_api_type** | Option<**String**> | Cluster access type. options: [native_k8s, rancher] | [optional][default to native_k8s]
**disable_issuer_validation** | Option<**String**> | Disable issuer validation [true/false] | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**k8s_auth_type** | Option<**String**> | K8S auth type [token/certificate]. (relevant for \"native_k8s\" only) | [optional][default to token]
**k8s_ca_cert** | Option<**String**> | The CA Certificate (base64 encoded) to use to call into the kubernetes API server | [optional]
**k8s_client_certificate** | Option<**String**> | Content of the k8 client certificate (PEM format) in a Base64 format (relevant for \"native_k8s\" only) | [optional]
**k8s_client_key** | Option<**String**> | Content of the k8 client private key (PEM format) in a Base64 format (relevant for \"native_k8s\" only) | [optional]
**k8s_host** | **String** | The URL of the kubernetes API server | 
**k8s_issuer** | Option<**String**> | The Kubernetes JWT issuer name. K8SIssuer is the claim that specifies who issued the Kubernetes token | [optional][default to kubernetes/serviceaccount]
**name** | **String** | K8S Auth config name | 
**new_name** | **String** | K8S Auth config new name | 
**rancher_api_key** | Option<**String**> | The api key used to access the TokenReview API to validate other JWTs (relevant for \"rancher\" only) | [optional]
**rancher_cluster_id** | Option<**String**> | The cluster id as define in rancher (relevant for \"rancher\" only) | [optional]
**signing_key** | **String** | The private key (base64 encoded) associated with the public key defined in the Kubernetes auth | 
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**token_exp** | Option<**i64**> | Time in seconds of expiration of the Akeyless Kube Auth Method token | [optional][default to 300]
**token_reviewer_jwt** | Option<**String**> | A Kubernetes service account JWT used to access the TokenReview API to validate other JWTs (relevant for \"native_k8s\" only). If not set, the JWT submitted in the authentication process will be used to access the Kubernetes TokenReview API. | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**use_gw_service_account** | Option<**bool**> | Use the GW's service account | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


