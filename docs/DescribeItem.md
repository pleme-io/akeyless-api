# DescribeItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accessibility** | Option<**String**> | for personal password manager | [optional][default to regular]
**bastion_details** | Option<**bool**> | Indicate if the item should return with ztb cluster details (url, etc) | [optional][default to false]
**der_certificate_format** | Option<**bool**> | The certificate will be displayed in DER format | [optional][default to false]
**display_id** | Option<**String**> | The display id of the item | [optional]
**gateway_details** | Option<**bool**> | Indicate if the item should return with clusters details (url, etc) | [optional][default to false]
**item_custom_fields_details** | Option<**bool**> | Include all item custom fields details | [optional][default to false]
**item_id** | Option<**i64**> | Item id of the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Item name | 
**services_details** | Option<**bool**> | Include all associated services details | [optional][default to false]
**show_versions** | Option<**bool**> | Include all item versions in reply | [optional][default to false]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


