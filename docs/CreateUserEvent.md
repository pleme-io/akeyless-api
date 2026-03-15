# CreateUserEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**capabilities** | Option<**Vec<String>**> | List of the required capabilities options: [read, update, delete,sra_transparently_connect]. Relevant only for request-access event types | [optional]
**comment** | Option<**String**> | Deprecated - use description | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**event_source** | Option<**String**> |  | [optional]
**event_type** | **String** |  | 
**item_name** | **String** | EventItemName Event item name | 
**item_type** | **String** | EventItemType Event item type | 
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**request_access_ttl** | Option<**i64**> | For how long to grant the requested access, in minutes | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


