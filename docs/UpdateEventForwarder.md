# UpdateEventForwarder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**admin_name** | Option<**String**> | Workstation Admin Name | [optional]
**auth_type** | Option<**String**> | The authentication type to use when connecting to ServiceNow (user-pass / jwt) | [optional][default to user-pass]
**client_id** | Option<**String**> | The client ID to use when connecting to ServiceNow with jwt authentication | [optional]
**description** | Option<**String**> | Description of the object | [optional][default to default_comment]
**email_to** | Option<**String**> | A comma seperated list of email addresses to send event to (relevant only for \"email\" Event Forwarder) | [optional]
**enable** | Option<**String**> | Enable/Disable Event Forwarder [true/false] | [optional][default to true]
**event_source_locations** | Option<**Vec<String>**> | Event sources | [optional]
**event_types** | Option<**Vec<String>**> | Event types | [optional]
**host** | Option<**String**> | Workstation Host | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | EventForwarder name | 
**new_comment** | Option<**String**> | Deprecated - use description | [optional][default to default_comment]
**new_name** | Option<**String**> | New EventForwarder name | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**user_email** | Option<**String**> | The user email to use when connecting to ServiceNow with jwt authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


