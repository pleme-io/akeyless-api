# GatewayUpdateDefaults

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cert_access_id** | Option<**String**> | Default Certificate access id for UI login | [optional][default to use-existing]
**event_on_status_change** | Option<**String**> | Trigger an event when Gateway status is changed [true/false] | [optional]
**hvp_route_version** | Option<**i64**> | Hvp route version to use [1/2] | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key** | Option<**String**> | The name of the gateway default encryption key | [optional][default to Default]
**oidc_access_id** | Option<**String**> | Default OIDC access id for UI login | [optional][default to use-existing]
**saml_access_id** | Option<**String**> | Default SAML access id for UI login | [optional][default to use-existing]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


