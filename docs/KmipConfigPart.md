# KmipConfigPart

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**clients** | Option<[**std::collections::HashMap<String, models::KmipClient>**](KMIPClient.md)> |  | [optional]
**key_enc** | Option<**Vec<i32>**> | Saves the private key of the cert issuer in encypted form | [optional]
**server** | Option<[**models::KmipServer**](KMIPServer.md)> |  | [optional]
**server_enc** | Option<**Vec<i32>**> | Saved for backward compatibility TODO: remove this after all clients upgrade | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


