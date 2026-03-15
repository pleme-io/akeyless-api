# CreateAuthMethodAzureAd

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_expires** | Option<**i64**> | Access expiration date in Unix timestamp (select 0 for access without expiry date) | [optional][default to 0]
**allowed_client_type** | Option<**Vec<String>**> | limit the auth method usage for specific client types [cli,ui,gateway-admin,sdk,mobile,extension] | [optional]
**audience** | Option<**String**> | Deprecated (Deprecated) The audience in the JWT | [optional][default to https://management.azure.com/]
**audit_logs_claims** | Option<**Vec<String>**> | Subclaims to include in audit logs, e.g \"--audit-logs-claims email --audit-logs-claims username\" | [optional]
**bound_group_id** | Option<**Vec<String>**> | A list of group ids that the access is restricted to | [optional]
**bound_ips** | Option<**Vec<String>**> | A CIDR whitelist with the IPs that the access is restricted to | [optional]
**bound_providers** | Option<**Vec<String>**> | A list of resource providers that the access is restricted to (e.g, Microsoft.Compute, Microsoft.ManagedIdentity, etc) | [optional]
**bound_resource_id** | Option<**Vec<String>**> | A list of full resource ids that the access is restricted to | [optional]
**bound_resource_names** | Option<**Vec<String>**> | A list of resource names that the access is restricted to (e.g, a virtual machine name, scale set name, etc). | [optional]
**bound_resource_types** | Option<**Vec<String>**> | A list of resource types that the access is restricted to (e.g, virtualMachines, userAssignedIdentities, etc) | [optional]
**bound_rg_id** | Option<**Vec<String>**> | A list of resource groups that the access is restricted to | [optional]
**bound_spid** | Option<**Vec<String>**> | A list of service principal IDs that the access is restricted to | [optional]
**bound_sub_id** | Option<**Vec<String>**> | A list of subscription ids that the access is restricted to | [optional]
**bound_tenant_id** | **String** | The Azure tenant id that the access is restricted to | 
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Auth Method description | [optional]
**expiration_event_in** | Option<**Vec<String>**> | How many days before the expiration of the auth method would you like to be notified. | [optional]
**force_sub_claims** | Option<**bool**> | if true: enforce role-association must include sub claims | [optional]
**gw_bound_ips** | Option<**Vec<String>**> | A CIDR whitelist with the GW IPs that the access is restricted to | [optional]
**issuer** | Option<**String**> | Issuer URL | [optional][default to https://sts.windows.net/---bound_tenant_id---]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**jwks_uri** | Option<**String**> | The URL to the JSON Web Key Set (JWKS) that containing the public keys that should be used to verify any JSON Web Token (JWT) issued by the authorization server. | [optional][default to https://login.microsoftonline.com/common/discovery/keys]
**jwt_ttl** | Option<**i64**> | Jwt TTL | [optional][default to 0]
**name** | **String** | Auth Method name | 
**product_type** | Option<**Vec<String>**> | Choose the relevant product type for the auth method [sm, sra, pm, dp, ca] | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**unique_identifier** | Option<**String**> | A unique identifier (ID) value which is a \"sub claim\" name that contains details uniquely identifying that resource. This \"sub claim\" is used to distinguish between different identities. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


