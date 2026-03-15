# KmipCreateClient

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**activate_keys_on_creation** | Option<**String**> | If set to 'true', newly created keys on the client will be set to an 'active' state | [optional][default to false]
**certificate_ttl** | Option<**i64**> | Client certificate TTL in days | [optional][default to 90]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Client name | 
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


