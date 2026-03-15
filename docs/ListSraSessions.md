# ListSraSessions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**resource_type** | Option<**Vec<String>**> | session resource type. In case it is empty, all resources type will be returned. options: [mysql, k8s, ssh, mongodb, mssql, postgres, aws, eks, gke, rdp] | [optional]
**status_type** | Option<**Vec<String>**> | session status type. In case it is empty, only active sessions will be returned. options: [connecting, connected, failed, completed, terminated] | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


