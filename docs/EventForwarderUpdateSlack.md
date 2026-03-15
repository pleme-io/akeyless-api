# EventForwarderUpdateSlack

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auth_methods_event_source_locations** | Option<**Vec<String>**> | Auth Method Event sources | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**enable** | Option<**String**> | Enable/Disable Event Forwarder [true/false] | [optional][default to true]
**event_types** | Option<**Vec<String>**> | List of event types to notify about [request-access, certificate-pending-expiration, certificate-expired, certificate-provisioning-success, certificate-provisioning-failure, auth-method-pending-expiration, auth-method-expired, next-automatic-rotation, rotated-secret-success, rotated-secret-failure, dynamic-secret-failure, multi-auth-failure, uid-rotation-failure, apply-justification, email-auth-method-approved, usage, rotation-usage, gateway-inactive, static-secret-updated, rate-limiting, usage-report, secret-sync] | [optional]
**gateways_event_source_locations** | **Vec<String>** | Event sources | 
**items_event_source_locations** | Option<**Vec<String>**> | Items Event sources | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**keep_prev_version** | Option<**String**> | Whether to keep previous version [true/false]. If not set, use default according to account settings | [optional]
**key** | Option<**String**> | The name of a key that used to encrypt the EventForwarder secret value (if empty, the account default protectionKey key will be used) | [optional]
**name** | **String** | EventForwarder name | 
**new_name** | Option<**String**> | New EventForwarder name | [optional]
**targets_event_source_locations** | Option<**Vec<String>**> | Targets Event sources | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**url** | **String** | Slack Webhook URL | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


