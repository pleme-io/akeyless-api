# TargetUpdateGke

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Description of the object | [optional]
**gke_account_key** | Option<**String**> | GKE Service Account key file path | [optional]
**gke_cluster_cert** | Option<**String**> | GKE cluster CA certificate | [optional]
**gke_cluster_endpoint** | Option<**String**> | GKE cluster URL endpoint | [optional]
**gke_cluster_name** | Option<**String**> | GKE cluster name | [optional]
**gke_service_account_email** | Option<**String**> | GKE service account email | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**keep_prev_version** | Option<**String**> | Whether to keep previous version [true/false]. If not set, use default according to account settings | [optional]
**key** | Option<**String**> | The name of a key that used to encrypt the target secret value (if empty, the account default protectionKey key will be used) | [optional]
**max_versions** | Option<**String**> | Set the maximum number of versions, limited by the account settings defaults. | [optional]
**name** | **String** | Target name | 
**new_name** | Option<**String**> | New target name | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**use_gw_cloud_identity** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


