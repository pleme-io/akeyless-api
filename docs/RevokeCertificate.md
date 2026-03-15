# RevokeCertificate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_id** | Option<**i64**> | The item id of the certificate to revoke | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | Option<**String**> | Certificate item name to revoke | [optional]
**serial_number** | Option<**String**> | The serial number of the certificate to revoke | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**version** | Option<**i32**> | Certificate version to revoke. Required if item-id or name are used. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


