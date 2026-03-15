# ValidateCertificateChallenge

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**result** | Option<[**models::ValidateCertificateChallengeOutput**](ValidateCertificateChallengeOutput.md)> |  | [optional]
**cert_display_id** | Option<**String**> | Certificate display ID from Phase 1 | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | Option<**String**> | Certificate name (alternative to cert-display-id) | [optional]
**timeout** | Option<**i64**> | Validation timeout in seconds | [optional][default to 120]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


