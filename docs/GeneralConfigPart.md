# GeneralConfigPart

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**akeyless_url** | Option<**String**> | AkeylessUrl is here for BC only. Gator will still return it if it exists in the configuration, but new clients (>=2.34.0) will ignore it and override it with what exists in their local file. It will no longer be sent to Gator for update, so new clusters will only have the default value saved in the DB. | [optional]
**api_token_ttl** | Option<**String**> |  | [optional]
**display_name** | Option<**String**> |  | [optional]
**enable_sni_proxy** | Option<**bool**> |  | [optional]
**enable_tls** | Option<**bool**> |  | [optional]
**enable_tls_configure** | Option<**bool**> |  | [optional]
**enable_tls_curl** | Option<**bool**> |  | [optional]
**enable_tls_hvp** | Option<**bool**> |  | [optional]
**gw_cluster_url** | Option<**String**> |  | [optional]
**hvp_route_version** | Option<**i64**> |  | [optional]
**notify_on_status_change** | Option<**bool**> |  | [optional]
**tcp_port** | Option<**String**> |  | [optional]
**tls_cert** | Option<**String**> |  | [optional]
**tls_cert_common_name** | Option<**String**> |  | [optional]
**tls_cert_expiration_date** | Option<**String**> |  | [optional]
**tls_cert_expiration_events** | Option<[**Vec<models::CertificateExpirationEvent>**](CertificateExpirationEvent.md)> |  | [optional]
**tls_key** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


