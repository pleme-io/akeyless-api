# GatewayUpdateTlsCert

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cert_data** | Option<**String**> | TLS Certificate (base64 encoded) | [optional]
**expiration_event_in** | Option<**Vec<String>**> | How many days before the expiration of the certificate would you like to be notified. | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key_data** | Option<**String**> | TLS Private Key (base64 encoded) | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


