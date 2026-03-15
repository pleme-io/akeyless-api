# EventForwarderUpdateServiceNow

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**admin_name** | Option<**String**> | Workstation Admin Name | [optional]
**admin_pwd** | Option<**String**> | Workstation Admin Password | [optional]
**app_private_key_base64** | Option<**String**> | The RSA Private Key to use when connecting with jwt authentication | [optional]
**auth_methods_event_source_locations** | Option<**Vec<String>**> | Auth Method Event sources | [optional]
**auth_type** | Option<**String**> | The authentication type to use [user-pass/jwt] | [optional][default to user-pass]
**client_id** | Option<**String**> | The client ID to use when connecting with jwt authentication | [optional]
**client_secret** | Option<**String**> | The client secret to use when connecting with jwt authentication | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**enable** | Option<**String**> | Enable/Disable Event Forwarder [true/false] | [optional][default to true]
**event_types** | Option<**Vec<String>**> | List of event types to notify about [request-access, certificate-pending-expiration, certificate-expired, certificate-provisioning-success, certificate-provisioning-failure, auth-method-pending-expiration, auth-method-expired, next-automatic-rotation, rotated-secret-success, rotated-secret-failure, dynamic-secret-failure, multi-auth-failure, uid-rotation-failure, apply-justification, email-auth-method-approved, usage, rotation-usage, gateway-inactive, static-secret-updated, rate-limiting, usage-report, secret-sync] | [optional]
**gateways_event_source_locations** | **Vec<String>** | Event sources | 
**host** | Option<**String**> | Workstation Host | [optional]
**items_event_source_locations** | Option<**Vec<String>**> | Items Event sources | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**keep_prev_version** | Option<**String**> | Whether to keep previous version [true/false]. If not set, use default according to account settings | [optional]
**key** | Option<**String**> | The name of a key that used to encrypt the EventForwarder secret value (if empty, the account default protectionKey key will be used) | [optional]
**name** | **String** | EventForwarder name | 
**new_name** | Option<**String**> | New EventForwarder name | [optional]
**targets_event_source_locations** | Option<**Vec<String>**> | Targets Event sources | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**user_email** | Option<**String**> | The user email to identify with when connecting with jwt authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


