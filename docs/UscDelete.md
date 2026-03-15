# UscDelete

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**force_delete** | Option<**bool**> | Force delete objects that are soft deleted by default (relavent only for Azure target) | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**namespace** | Option<**String**> | The namespace (relevant for Hashi vault target) | [optional]
**secret_id** | **String** | The universal secrets id (or name, for AWS, Azure, K8s or Hashi vault targets) to delete | 
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**usc_name** | **String** | Name of the Universal Secrets Connector item | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


