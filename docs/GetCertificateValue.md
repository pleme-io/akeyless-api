# GetCertificateValue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cert_issuer_name** | Option<**String**> | The parent PKI Certificate Issuer's name of the certificate, required when used with display-id and token | [optional]
**display_id** | Option<**String**> | Certificate display ID | [optional]
**ignore_cache** | Option<**String**> | Retrieve the Secret value without checking the Gateway's cache [true/false]. This flag is only relevant when using the RestAPI | [optional][default to false]
**issuance_token** | Option<**String**> | Token for getting the issued certificate | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | Option<**String**> | Certificate name | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**version** | Option<**i32**> | Certificate version | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


