# UpdateEksTarget

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**comment** | Option<**String**> | Deprecated - use description | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**eks_access_key_id** | **String** | Access Key ID | 
**eks_cluster_ca_cert** | **String** | EKS cluster CA certificate | 
**eks_cluster_endpoint** | **String** | EKS cluster URL endpoint | 
**eks_cluster_name** | **String** | EKS cluster name | 
**eks_region** | Option<**String**> | Region | [optional][default to us-east-2]
**eks_secret_access_key** | **String** | Secret Access Key | 
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**keep_prev_version** | Option<**String**> | Whether to keep previous version [true/false]. If not set, use default according to account settings | [optional]
**key** | Option<**String**> | The name of a key that used to encrypt the target secret value (if empty, the account default protectionKey key will be used) | [optional]
**max_versions** | Option<**String**> | Set the maximum number of versions, limited by the account settings defaults. | [optional]
**name** | **String** | Target name | 
**new_name** | Option<**String**> | New target name | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**update_version** | Option<**bool**> | Deprecated | [optional]
**use_gw_cloud_identity** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


