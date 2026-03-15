# UidCreateChildToken

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auth_method_name** | Option<**String**> | The universal identity auth method name, required only when uid-token is not provided | [optional]
**child_deny_inheritance** | Option<**bool**> | Deny from new child to create their own children | [optional]
**child_deny_rotate** | Option<**bool**> | Deny from new child to rotate | [optional]
**child_ttl** | Option<**i32**> | New child token ttl | [optional]
**comment** | Option<**String**> | Deprecated - use description | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**uid_token_id** | Option<**String**> | The ID of the uid-token, required only when uid-token is not provided | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


