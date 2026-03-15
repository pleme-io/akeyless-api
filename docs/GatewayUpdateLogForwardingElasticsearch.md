# GatewayUpdateLogForwardingElasticsearch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_key** | Option<**String**> | Elasticsearch api key relevant only for api_key auth-type | [optional]
**auth_type** | Option<**String**> | Elasticsearch auth type [api_key/password] | [optional]
**cloud_id** | Option<**String**> | Elasticsearch cloud id relevant only for cloud server-type | [optional]
**enable** | Option<**String**> | Enable Log Forwarding [true/false] | [optional][default to true]
**enable_tls** | Option<**bool**> | Enable tls | [optional]
**index** | Option<**String**> | Elasticsearch index | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**nodes** | Option<**String**> | Elasticsearch nodes relevant only for nodes server-type | [optional]
**output_format** | Option<**String**> | Logs format [text/json] | [optional][default to text]
**password** | Option<**String**> | Elasticsearch password relevant only for password auth-type | [optional]
**pull_interval** | Option<**String**> | Pull interval in seconds | [optional][default to 10]
**server_type** | Option<**String**> | Elasticsearch server type [cloud/nodes] | [optional]
**tls_certificate** | Option<**String**> | Elasticsearch tls certificate | [optional][default to use-existing]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**user_name** | Option<**String**> | Elasticsearch user name relevant only for password auth-type | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


