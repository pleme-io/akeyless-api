# GatewayUpdateLogForwardingSplunk

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enable** | Option<**String**> | Enable Log Forwarding [true/false] | [optional][default to true]
**enable_batch** | Option<**String**> | Enable batch forwarding [true/false] | [optional][default to true]
**enable_tls** | Option<**bool**> | Enable tls | [optional]
**index** | Option<**String**> | Splunk index | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**output_format** | Option<**String**> | Logs format [text/json] | [optional][default to text]
**pull_interval** | Option<**String**> | Pull interval in seconds | [optional][default to 10]
**source** | Option<**String**> | Splunk source | [optional][default to use-existing]
**source_type** | Option<**String**> | Splunk source type | [optional][default to use-existing]
**splunk_token** | Option<**String**> | Splunk token | [optional]
**splunk_url** | Option<**String**> | Splunk server URL | [optional]
**tls_certificate** | Option<**String**> | Splunk tls certificate | [optional][default to use-existing]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


