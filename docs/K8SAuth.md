# K8SAuth

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**am_token_expiration** | Option<**i64**> | AuthMethodTokenExpiration is time in seconds of expiration of the Akeyless Kube Auth Method token | [optional]
**auth_method_access_id** | Option<**String**> | AuthMethodAccessId of the Kubernetes auth method | [optional]
**auth_method_prv_key_pem** | Option<**String**> | AuthMethodSigningKey is the private key (in base64 of the PEM format) associated with the public key defined in the Kubernetes auth method, that used to sign the internal token for the Akeyless Kubernetes Auth Method | [optional]
**cluster_api_type** | Option<**String**> | ClusterApiType defines types of API access to cluster | [optional]
**disable_iss_validation** | Option<**bool**> | DisableISSValidation is optional parameter to disable ISS validation | [optional]
**id** | Option<**String**> |  | [optional]
**k8s_auth_type** | Option<**String**> |  | [optional]
**k8s_ca_cert** | Option<**String**> | K8SCACert is the CA Cert to use to call into the kubernetes API | [optional]
**k8s_client_cert_data** | Option<**String**> | K8sClientCertData is the client certificate for k8s client certificate authentication | [optional]
**k8s_client_key_data** | Option<**String**> | K8sClientKeyData is the client key for k8s client certificate authentication | [optional]
**k8s_host** | Option<**String**> | K8SHost is the url string for the kubernetes API | [optional]
**k8s_issuer** | Option<**String**> | K8SIssuer is the claim that specifies who issued the Kubernetes token | [optional]
**k8s_pub_keys_pem** | Option<**Vec<String>**> | K8SPublicKeysPEM is the list of public key in PEM format | [optional]
**k8s_token_reviewer_jwt** | Option<**String**> | K8STokenReviewerJWT is the bearer for clusterApiTypeK8s, used during TokenReview API call | [optional]
**name** | Option<**String**> |  | [optional]
**rancher_api_key** | Option<**String**> | RancherApiKey the bear token for clusterApiTypeRancher | [optional]
**rancher_cluster_id** | Option<**String**> | RancherClusterId cluster id as define in rancher (in case of clusterApiTypeRancher) | [optional]
**use_local_ca_jwt** | Option<**bool**> | UseLocalCAJwt is an optional parameter to set defaulting to using the local service account when running in a Kubernetes pod | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


