# UpdateSalesforceTarget

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**app_private_key_data** | Option<**String**> | Base64 encoded PEM of the connected app private key (relevant for JWT auth only) | [optional]
**auth_flow** | **String** | type of the auth flow ('jwt' / 'user-password') | 
**ca_cert_data** | Option<**String**> | Base64 encoded PEM cert to use when uploading a new key to Salesforce | [optional]
**ca_cert_name** | Option<**String**> | name of the certificate in Salesforce tenant to use when uploading new key | [optional]
**client_id** | **String** | Client ID of the oauth2 app to use for connecting to Salesforce | 
**client_secret** | Option<**String**> | Client secret of the oauth2 app to use for connecting to Salesforce (required for password flow) | [optional]
**comment** | Option<**String**> | Deprecated - use description | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**email** | **String** | The email of the user attached to the oauth2 app used for connecting to Salesforce | 
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**keep_prev_version** | Option<**String**> | Whether to keep previous version [true/false]. If not set, use default according to account settings | [optional]
**key** | Option<**String**> | The name of a key that used to encrypt the target secret value (if empty, the account default protectionKey key will be used) | [optional]
**max_versions** | Option<**String**> | Set the maximum number of versions, limited by the account settings defaults. | [optional]
**name** | **String** | Target name | 
**new_name** | Option<**String**> | New target name | [optional]
**password** | Option<**String**> | The password of the user attached to the oauth2 app used for connecting to Salesforce (required for user-password flow) | [optional]
**security_token** | Option<**String**> | The security token of the user attached to the oauth2 app used for connecting to Salesforce  (required for user-password flow) | [optional]
**tenant_url** | **String** | Url of the Salesforce tenant | 
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**update_version** | Option<**bool**> | Deprecated | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


