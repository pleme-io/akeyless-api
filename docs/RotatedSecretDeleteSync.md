# RotatedSecretDeleteSync

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delete_from_usc** | Option<**bool**> | Delete the secret from the remote target USC as well | [optional][default to false]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Rotated secret name | 
**remote_secret_name** | Option<**String**> | Remote Secret Name to disambiguate when multiple syncs exist under the same USC | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**usc_name** | **String** | Universal Secret Connector name | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


