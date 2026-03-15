# SetRoleRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**capability** | **Vec<String>** | List of the approved/denied capabilities in the path options: [read, create, update, delete, list, deny] | 
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**path** | **String** | The path the rule refers to | 
**role_name** | **String** | The role name to be updated | 
**rule_type** | Option<**String**> | item-rule, target-rule, role-rule, auth-method-rule, search-rule, reports-rule, gw-reports-rule or sra-reports-rule, sra-rule | [optional][default to item-rule]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**ttl** | Option<**i32**> | RoleRule ttl | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


