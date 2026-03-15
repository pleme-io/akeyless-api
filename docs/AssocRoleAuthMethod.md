# AssocRoleAuthMethod

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**am_name** | **String** | The auth method to associate | 
**case_sensitive** | Option<**String**> | Treat sub claims as case-sensitive [true/false] | [optional][default to true]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**role_name** | **String** | The role to associate | 
**sub_claims** | Option<**std::collections::HashMap<String, String>**> | key/val of sub claims, e.g group=admins,developers | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


