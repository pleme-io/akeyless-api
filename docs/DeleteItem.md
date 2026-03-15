# DeleteItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accessibility** | Option<**String**> | for personal password manager | [optional][default to regular]
**delete_immediately** | Option<**bool**> | When delete-in-days=-1, must be set | [optional][default to false]
**delete_in_days** | Option<**i64**> | The number of days to wait before deleting the item (relevant for keys only) | [optional][default to 7]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Item name | 
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**version** | Option<**i32**> | The specific version you want to delete - 0=last version, -1=entire item with all versions (default) | [optional][default to -1]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


