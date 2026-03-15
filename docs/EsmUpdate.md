# EsmUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**binary_value** | Option<**bool**> | Use this option if the external secret value is a base64 encoded binary | [optional]
**description** | Option<**String**> | Description of the external secret | [optional]
**esm_name** | **String** | Name of the External Secrets Manager item | 
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**secret_id** | **String** | The external secret id (or name, for AWS, Azure or K8s targets) to update | 
**tags** | Option<**std::collections::HashMap<String, String>**> | Tags for the external secret | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**value** | **String** | Value of the external secret item, either text or base64 encoded binary | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


