# TargetCreateGlobalSignAtlas

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_key** | **String** | API Key of the GlobalSign Atlas account | 
**api_secret** | **String** | API Secret of the GlobalSign Atlas account | 
**description** | Option<**String**> | Description of the object | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key** | Option<**String**> | The name of a key that used to encrypt the target secret value (if empty, the account default protectionKey key will be used) | [optional]
**max_versions** | Option<**String**> | Set the maximum number of versions, limited by the account settings defaults. | [optional]
**mtls_cert_data_base64** | Option<**String**> | Mutual TLS Certificate contents of the GlobalSign Atlas account encoded in base64, either mtls-cert-file-path or mtls-cert-data-base64 must be supplied | [optional]
**mtls_key_data_base64** | Option<**String**> | Mutual TLS Key contents of the GlobalSign Atlas account encoded in base64, either mtls-key-file-path or mtls-data-base64 must be supplied | [optional]
**name** | **String** | Target name | 
**timeout** | Option<**String**> | Timeout waiting for certificate validation in Duration format (1h - 1 Hour, 20m - 20 Minutes, 33m3s - 33 Minutes and 3 Seconds), maximum 1h. | [optional][default to 5m]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


