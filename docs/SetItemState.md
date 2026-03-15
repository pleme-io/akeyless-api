# SetItemState

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**desired_state** | **String** | Desired item state (Enabled, Disabled) | 
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Current item name | 
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**version** | Option<**i32**> | The specific version you want to update: 0=item level state (default) (relevant only for keys) | [optional][default to 0]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


