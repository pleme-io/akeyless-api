# GatewayCreateProducerDockerhub

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**dockerhub_password** | Option<**String**> | DockerhubPassword is either the user's password access token to manage the repository | [optional]
**dockerhub_token_scopes** | Option<**String**> | Access token scopes list (comma-separated) to give the dynamic secret valid options are in \"repo:admin\", \"repo:write\", \"repo:read\", \"repo:public_read\" | [optional]
**dockerhub_username** | Option<**String**> | DockerhubUsername is the name of the user in dockerhub | [optional]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Dynamic secret name | 
**producer_encryption_key_name** | Option<**String**> | Dynamic producer encryption key | [optional]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**target_name** | Option<**String**> | Target name | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**user_ttl** | Option<**String**> | User TTL | [optional][default to 60m]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


