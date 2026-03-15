# AzureAdAccessRules

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ad_endpoint** | Option<**String**> | The audience in the JWT. | [optional]
**azure_cloud** | Option<**String**> | Azure cloud environment [AzureCloud/AzureUSGovernment/AzureChinaCloud]. For create/update, cloud is inferred from jwks_uri. | [optional]
**bound_group_ids** | Option<**Vec<String>**> | The list of group ids that login is restricted to. | [optional]
**bound_resource_groups** | Option<**Vec<String>**> | The list of resource groups that login is restricted to. | [optional]
**bound_resource_ids** | Option<**Vec<String>**> | The list of full resource ids that the login is restricted to. | [optional]
**bound_resource_names** | Option<**Vec<String>**> | The list of resource names that the login is restricted to (e.g, a virtual machine name, scale set name, etc). | [optional]
**bound_resource_providers** | Option<**Vec<String>**> | The list of resource providers that login is restricted to (e.g, Microsoft.Compute, Microsoft.ManagedIdentity, etc). | [optional]
**bound_resource_types** | Option<**Vec<String>**> | The list of resource types that login is restricted to  (e.g, virtualMachines, userAssignedIdentities, etc). | [optional]
**bound_service_principal_ids** | Option<**Vec<String>**> | The list of service principal IDs that login is restricted to. | [optional]
**bound_subscription_ids** | Option<**Vec<String>**> | The list of subscription IDs that login is restricted to. | [optional]
**bound_tenant_id** | Option<**String**> | The tenants id for the Azure Active Directory organization. | [optional]
**issuer** | Option<**String**> | Issuer URL | [optional]
**jwks_uri** | Option<**String**> | The URL to the JSON Web Key Set (JWKS) that containing the public keys that should be used to verify any JSON Web Token (JWT) issued by the authorization server. | [optional]
**unique_identifier** | Option<**String**> | A unique identifier to distinguish different users | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


