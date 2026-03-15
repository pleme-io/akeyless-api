# CreateEsm

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**azure_kv_name** | Option<**String**> | Azure Key Vault name (Relevant only for Azure targets) | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the External Secrets Manager | [optional]
**gcp_project_id** | Option<**String**> | GCP Project ID (Relevant only for GCP targets) | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**k8s_namespace** | Option<**String**> | K8s namespace (Relevant to Kubernetes targets) | [optional]
**name** | **String** | External Secrets Manager name | 
**tags** | Option<**Vec<String>**> | List of the tags attached to this External Secrets Manager | [optional]
**target_to_associate** | **String** | Target External Secrets Manager to connect | 
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


