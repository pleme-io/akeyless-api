# GwUpdateRemoteAccessSessionLogsLogstash

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dns** | Option<**String**> | Logstash dns | [optional]
**enable** | Option<**String**> | Enable Log Forwarding [true/false] | [optional][default to true]
**enable_tls** | Option<**bool**> | Enable tls | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**output_format** | Option<**String**> | Logs format [text/json] | [optional][default to text]
**protocol** | Option<**String**> | Logstash protocol [tcp/udp] | [optional]
**pull_interval** | Option<**String**> | Pull interval in seconds | [optional][default to 10]
**tls_certificate** | Option<**String**> | Logstash tls certificate | [optional][default to use-existing]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


