# CreateEventForwarder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**admin_name** | Option<**String**> | Workstation Admin Name | [optional]
**admin_pwd** | Option<**String**> | Workstation Admin password | [optional]
**app_private_key_base64** | Option<**String**> | The RSA Private Key PEM formatted in base64 to use when connecting to ServiceNow with jwt authentication | [optional]
**auth_type** | Option<**String**> | The authentication type to use when connecting to ServiceNow (user-pass / jwt) | [optional][default to user-pass]
**client_id** | Option<**String**> | The client ID to use when connecting to ServiceNow with jwt authentication | [optional]
**client_secret** | Option<**String**> | The client secret to use when connecting to ServiceNow with jwt authentication | [optional]
**comment** | Option<**String**> | Deprecated - use description | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**email_to** | Option<**String**> | A comma seperated list of email addresses to send event to (relevant only for \"email\" Event Forwarder) | [optional]
**event_source_locations** | **Vec<String>** | Event sources | 
**event_source_type** | Option<**String**> | Event Source type [item, target, auth_method, gateway] | [optional][default to item]
**event_types** | Option<**Vec<String>**> | List of event types to notify about [request-access, certificate-pending-expiration, certificate-expired, certificate-provisioning-success, certificate-provisioning-failure, auth-method-pending-expiration, auth-method-expired, rotated-secret-success, rotated-secret-failure, dynamic-secret-failure, multi-auth-failure, uid-rotation-failure, apply-justification, email-auth-method-approved, usage, rotation-usage, gateway-inactive, static-secret-updated] | [optional]
**every** | Option<**String**> | Rate of periodic runner repetition in hours | [optional]
**forwarder_type** | **String** |  | 
**host** | Option<**String**> | Workstation Host | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key** | Option<**String**> | The name of a key that used to encrypt the EventForwarder secret value (if empty, the account default protectionKey key will be used) | [optional]
**name** | **String** | EventForwarder name | 
**runner_type** | **String** |  | 
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**user_email** | Option<**String**> | The user email to use when connecting to ServiceNow with jwt authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


