# GwUpdateRemoteAccessSessionLogsGoogleChronicle

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**customer_id** | Option<**String**> | Google chronicle customer id | [optional]
**enable** | Option<**String**> | Enable Log Forwarding [true/false] | [optional][default to true]
**gcp_key** | Option<**String**> | Base64-encoded service account private key text | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**log_type** | Option<**String**> | Google chronicle log type | [optional]
**output_format** | Option<**String**> | Logs format [text/json] | [optional][default to text]
**pull_interval** | Option<**String**> | Pull interval in seconds | [optional][default to 10]
**region** | Option<**String**> | Google chronicle region [eu_multi_region/london/us_multi_region/singapore/tel_aviv] | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


