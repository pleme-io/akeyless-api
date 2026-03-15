# UidRevokeToken

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auth_method_name** | Option<**String**> | The universal identity auth method name | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**revoke_token** | **String** | the universal identity token/token-id to revoke | 
**revoke_type** | **String** | revokeSelf/revokeAll (delete only this token/this token and his children) | 
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


