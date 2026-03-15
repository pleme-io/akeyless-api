# TargetUpdateAzure

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**azure_cloud** | Option<**String**> | Azure cloud environment to use. Values: AzureCloud (default), AzureUSGovernment, AzureChinaCloud. | [optional][default to AzureCloud]
**client_id** | Option<**String**> | Azure client/application id | [optional]
**client_secret** | Option<**String**> | Azure client secret | [optional]
**connection_type** | Option<**String**> | Type of connection [credentials/cloud-identity] | [optional][default to credentials]
**description** | Option<**String**> | Description of the object | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**keep_prev_version** | Option<**String**> | Whether to keep previous version [true/false]. If not set, use default according to account settings | [optional]
**key** | Option<**String**> | The name of a key that used to encrypt the target secret value (if empty, the account default protectionKey key will be used) | [optional]
**max_versions** | Option<**String**> | Set the maximum number of versions, limited by the account settings defaults. | [optional]
**name** | **String** | Target name | 
**new_name** | Option<**String**> | New target name | [optional]
**resource_group_name** | Option<**String**> | The Resource Group name in your Azure subscription | [optional]
**resource_name** | Option<**String**> | The name of the relevant Resource | [optional]
**subscription_id** | Option<**String**> | Azure Subscription Id | [optional]
**tenant_id** | Option<**String**> | Azure tenant id | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**use_gw_cloud_identity** | Option<**bool**> | Use the GW's Cloud IAM [Deprecated: Use connection-type=cloud-identity] | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


