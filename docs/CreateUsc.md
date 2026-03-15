# CreateUsc

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**azure_kv_name** | Option<**String**> | Azure Key Vault name (Relevant only for Azure targets) | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the Universal Secrets Connector | [optional]
**environment_names** | Option<**String**> | The environments in repo-name/environment-name format, comma-separated (only relevant for: github-scope=repository-environment) | [optional]
**gcp_project_id** | Option<**String**> | GCP Project ID (Relevant only for GCP targets) | [optional]
**gcp_sm_regions** | Option<**String**> | GCP Secret Manager regions to query for regional secrets (comma-separated, e.g., us-east1,us-west1). Max 12 regions. Required when listing with object-type=regional-secrets. | [optional]
**github_scope** | Option<**String**> | The scope where secrets will be created, available options: [repository, organization, repository-environment] | [optional][default to repository]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**k8s_namespace** | Option<**String**> | K8s namespace (Relevant to Kubernetes targets) | [optional]
**name** | **String** | Universal Secrets Connector name | 
**organization_name** | Option<**String**> | The organization name to create the secret in (only relevant for: github-scope=organization) | [optional]
**repository_access** | Option<**String**> |  | [optional][default to public]
**repository_names** | Option<**String**> | The repository names, comma-separated (only relevant for: github-scope=repository) | [optional]
**tags** | Option<**Vec<String>**> | List of the tags attached to this Universal Secrets Connector | [optional]
**target_to_associate** | **String** | Target Universal Secrets Connector to connect | 
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**usc_prefix** | Option<**String**> | Prefix for all secrets created in AWS Secrets Manager | [optional]
**use_prefix_as_filter** | Option<**String**> | Whether to filter the USC secret list using the specified usc-prefix [true/false] | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


