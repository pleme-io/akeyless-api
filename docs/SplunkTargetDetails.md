# SplunkTargetDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**audience** | Option<**String**> | Token audience | [optional]
**auth_mode** | Option<**String**> | Authentication mode: \"username\" or \"token\" | [optional]
**password** | Option<**String**> |  | [optional]
**splunk_payload** | Option<[**models::SplunkPayload**](SplunkPayload.md)> |  | [optional]
**splunk_url** | Option<**String**> | Splunk server URL | [optional]
**token** | Option<**String**> | Token is used when AuthMode == \"token\" | [optional]
**token_owner** | Option<**String**> | Token owner (the Splunk user who owns the token, required for token rotation) | [optional]
**use_tls** | Option<**bool**> | Use TLS certificate verification when connecting to the Splunk management API. | [optional]
**username** | Option<**String**> | Username & Password are used when AuthMode == \"username\" | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


