# GatewayUpdateLogForwardingDatadog

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_key** | Option<**String**> | Datadog api key | [optional]
**enable** | Option<**String**> | Enable Log Forwarding [true/false] | [optional][default to true]
**host** | Option<**String**> | Datadog host | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**log_service** | Option<**String**> | Datadog log service | [optional][default to use-existing]
**log_source** | Option<**String**> | Datadog log source | [optional][default to use-existing]
**log_tags** | Option<**String**> | A comma-separated list of Datadog log tags formatted as \"key:value\" strings | [optional][default to use-existing]
**output_format** | Option<**String**> | Logs format [text/json] | [optional][default to text]
**pull_interval** | Option<**String**> | Pull interval in seconds | [optional][default to 10]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


