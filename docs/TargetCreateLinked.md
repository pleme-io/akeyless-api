# TargetCreateLinked

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Description of the object | [optional]
**hosts** | Option<**String**> | A comma seperated list of server hosts and server descriptions joined by semicolon ';' (i.e. 'server-dev.com;My Dev server,server-prod.com;My Prod server description') | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Target name | 
**parent_target_name** | Option<**String**> | The parent Target name | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**r#type** | Option<**String**> | Specifies the hosts type, relevant only when working without parent target | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


