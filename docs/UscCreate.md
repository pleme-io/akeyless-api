# UscCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**binary_value** | Option<**bool**> | Use this option if the universal secrets value is a base64 encoded binary | [optional]
**description** | Option<**String**> | Description of the universal secrets | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**namespace** | Option<**String**> | The namespace (relevant for Hashi vault target) | [optional]
**object_type** | Option<**String**> |  | [optional]
**pfx_password** | Option<**String**> | Optional, the passphrase that protects the private key within the pfx certificate (Relevant only for Azure KV certificates) | [optional]
**region** | Option<**String**> | Optional, create secret in a specific region (GCP only). If empty, a global secret will be created (provider default). | [optional]
**secret_name** | **String** | Name for the new universal secrets | 
**tags** | Option<**std::collections::HashMap<String, String>**> | Tags for the universal secrets | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**usc_encryption_key** | Option<**String**> | Optional, The name of the remote key that used to encrypt the secret value (if empty, the default key will be used) | [optional]
**usc_name** | **String** | Name of the Universal Secrets Connector item | 
**value** | **String** | Value of the universal secrets item, either text or base64 encoded binary | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


