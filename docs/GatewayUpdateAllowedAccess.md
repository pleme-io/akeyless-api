# GatewayUpdateAllowedAccess

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sub_claims_case_insensitive** | Option<**bool**> |  | [optional]
**access_id** | **String** | Access ID The access id to be attached to this allowed access. Auth method with this access id should already exist. | 
**case_sensitive** | Option<**String**> | Treat sub claims as case-sensitive [true/false] | [optional][default to true]
**description** | Option<**String**> | Allowed access description | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Allowed access name | 
**new_name** | Option<**String**> | New allowed access name | [optional]
**permissions** | Option<**String**> | Permissions  Comma-seperated list of permissions for this allowed access. Available permissions: [defaults,targets,classic_keys,automatic_migration,ldap_auth,dynamic_secret,k8s_auth,log_forwarding,zero_knowledge_encryption,rotated_secret,caching,event_forwarding,admin,kmip,general,rotate_secret_value] | [optional]
**sub_claims** | Option<**std::collections::HashMap<String, String>**> | Sub claims key/val of sub claims, e.g group=admins,developers | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


