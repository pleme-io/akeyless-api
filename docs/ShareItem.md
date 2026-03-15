# ShareItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accessibility** | Option<**String**> | for personal password manager | [optional][default to regular]
**action** | **String** | Action to be performed on the item [start/stop/describe] | 
**emails** | Option<**Vec<String>**> | List of emails to start/stop sharing the secret with | [optional]
**item_name** | **String** | Item name | 
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**share_type** | Option<**String**> | Share type [email/token] | [optional][default to email]
**shared_token_id** | Option<**Vec<String>**> | Shared token ids in order to stop sharing a secret | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**ttl** | Option<**i32**> | TTL of the Availability of the shared secret in seconds | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**view_once** | Option<**bool**> | ViewOnlyOnce Shared secrets can only be viewed once [true/false] | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


