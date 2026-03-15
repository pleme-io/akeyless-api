# UpdateRotationSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auto_rotate** | **bool** | Whether to automatically rotate every --rotation-interval days, or disable existing automatic rotation | 
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Key name | 
**rotation_event_in** | Option<**Vec<String>**> | How many days before the rotation of the item would you like to be notified | [optional]
**rotation_interval** | Option<**i64**> | The number of days to wait between every automatic key rotation (7-365) | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


