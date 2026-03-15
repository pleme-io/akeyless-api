# UpdateRole

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**analytics_access** | Option<**String**> | Allow this role to view analytics. Currently only 'none', 'own', 'all' values are supported, allowing associated auth methods to view reports produced by the same auth methods. | [optional]
**audit_access** | Option<**String**> | Allow this role to view audit logs. Currently only 'none', 'own', 'scoped' and 'all' values are supported, allowing associated auth methods to view audit logs produced by the same auth methods. | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional][default to default_comment]
**event_center_access** | Option<**String**> | Allow this role to view Event Center. Currently only 'none', 'scoped' and 'all' values are supported | [optional]
**event_forwarder_access** | Option<**String**> | Allow this role to manage Event Forwarders. Currently only 'none' and 'all' values are supported. | [optional]
**gw_analytics_access** | Option<**String**> | Allow this role to view gw analytics. Currently only 'none', 'scoped', 'all' values are supported, allowing associated auth methods to view reports produced by the same auth methods. | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Role name | 
**new_comment** | Option<**String**> | Deprecated - use description | [optional][default to default_comment]
**new_name** | Option<**String**> | New Role name | [optional]
**reverse_rbac_access** | Option<**String**> | Allow this role to view Reverse RBAC. Supported values: 'scoped', 'all'. | [optional]
**sra_reports_access** | Option<**String**> | Allow this role to view SRA Clusters. Currently only 'none', 'scoped', 'all' values are supported. | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**usage_reports_access** | Option<**String**> | Allow this role to view Usage Report. Currently only 'none' and 'all' values are supported. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


