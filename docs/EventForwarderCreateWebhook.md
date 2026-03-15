# EventForwarderCreateWebhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auth_methods_event_source_locations** | Option<**Vec<String>**> | Auth Method Event sources | [optional]
**auth_token** | Option<**String**> | Base64 encoded Token string for authentication type Token | [optional]
**auth_type** | Option<**String**> | The Webhook authentication type [user-pass, bearer-token, certificate] | [optional][default to user-pass]
**client_cert_data** | Option<**String**> | Base64 encoded PEM certificate, relevant for certificate auth-type | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**event_types** | Option<**Vec<String>**> | List of event types to notify about [request-access, certificate-pending-expiration, certificate-expired, certificate-provisioning-success, certificate-provisioning-failure, auth-method-pending-expiration, auth-method-expired, next-automatic-rotation, rotated-secret-success, rotated-secret-failure, dynamic-secret-failure, multi-auth-failure, uid-rotation-failure, apply-justification, email-auth-method-approved, usage, rotation-usage, gateway-inactive, static-secret-updated, rate-limiting, usage-report, secret-sync] | [optional]
**every** | Option<**String**> | Rate of periodic runner repetition in hours | [optional]
**gateways_event_source_locations** | **Vec<String>** | Event sources | 
**items_event_source_locations** | Option<**Vec<String>**> | Items Event sources | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key** | Option<**String**> | The name of a key that used to encrypt the EventForwarder secret value (if empty, the account default protectionKey key will be used) | [optional]
**name** | **String** | EventForwarder name | 
**password** | Option<**String**> | Password for authentication relevant for user-pass auth-type | [optional]
**private_key_data** | Option<**String**> | Base64 encoded PEM RSA Private Key, relevant for certificate auth-type | [optional]
**runner_type** | **String** |  | 
**server_certificates** | Option<**String**> | Base64 encoded PEM certificate of the Webhook | [optional]
**targets_event_source_locations** | Option<**Vec<String>**> | Targets Event sources | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**url** | Option<**String**> | Webhook URL | [optional]
**username** | Option<**String**> | Username for authentication relevant for user-pass auth-type | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


