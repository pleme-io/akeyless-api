# GetLastUserEventStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_source** | Option<**String**> |  | [optional]
**event_type** | **String** |  | 
**item_name** | **String** | Event item name | 
**item_type** | **String** | Event item type can be either \"target\" or type of item eg \"static_secret\"/\"dynamic_secret\" To get type of some item run `akeyless describe-item -n {ITEM_NAME} --jq-expression .item_type` | 
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**time_back** | Option<**String**> | The time back to search the event, for example if the value is \"5m\" we will return the last user event issued in the last 5 minutes. By default, we will search without any time boundary. | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


