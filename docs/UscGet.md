# UscGet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**namespace** | Option<**String**> | The namespace (relevant for Hashi vault target) | [optional]
**secret_id** | **String** | The secret id (or name, for AWS, Azure, K8s or Hashi vault targets) to get from the Universal Secrets Connector | 
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**usc_name** | **String** | Name of the Universal Secrets Connector item | 
**version_id** | Option<**String**> | The version id (if not specified, will retrieve the last version) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


