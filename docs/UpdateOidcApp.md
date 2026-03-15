# UpdateOidcApp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**audience** | Option<**String**> | A comma separated list of allowed audiences | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key** | Option<**String**> | The name of a key that used to encrypt the OIDC application (if empty, the account default protectionKey key will be used) | [optional]
**name** | **String** | OIDC application name | 
**permission_assignment** | Option<**String**> | A json string defining the access permission assignment for this app | [optional]
**public** | Option<**bool**> | Set to true if the app is public (cannot keep secrets) | [optional]
**redirect_uris** | Option<**String**> | A comma separated list of allowed redirect uris | [optional]
**scopes** | Option<**String**> | A comma separated list of allowed scopes | [optional][default to openid]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


