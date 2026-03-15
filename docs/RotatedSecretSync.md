# RotatedSecretSync

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delete_remote** | Option<**bool**> | Delete the secret from remote secret manager (for association create/update) | [optional]
**filter_secret_value** | Option<**String**> | JQ expression to filter or transform the secret value | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Rotated secret name | 
**namespace** | Option<**String**> | Vault namespace, releavnt only for Hashicorp Vault Target | [optional]
**remote_secret_name** | Option<**String**> | Remote Secret Name that will be synced on the remote endpoint | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**usc_name** | Option<**String**> | Universal Secret Connector name, If not provided all attached USC's will be synced | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


