# TargetUpdateLinked

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**add_hosts** | Option<**String**> | A comma seperated list of new server hosts and server descriptions joined by semicolon ';' that will be added to the Linked Target hosts. | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**hosts** | Option<**String**> | A comma seperated list of server hosts and server descriptions joined by semicolon ';' (i.e. 'server-dev.com;My Dev server,server-prod.com;My Prod server description') | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**keep_prev_version** | Option<**String**> | Whether to keep previous version [true/false]. If not set, use default according to account settings | [optional]
**name** | **String** | Linked Target name | 
**new_name** | Option<**String**> | New Linked Target name | [optional]
**parent_target_name** | Option<**String**> | The parent Target name | [optional]
**rm_hosts** | Option<**String**> | Comma separated list of existing hosts that will be removed from Linked Target hosts. | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**r#type** | Option<**String**> | Specifies the hosts type, relevant only when working without parent target | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


