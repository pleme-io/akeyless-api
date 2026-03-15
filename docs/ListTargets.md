# ListTargets

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter by auth method name or part of it | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**pagination_token** | Option<**String**> | Next page reference | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**r#type** | Option<**Vec<String>**> | The target types list . In case it is empty, all types of targets will be returned. options: [hanadb cassandra aws ssh gke eks mysql mongodb snowflake mssql redshift artifactory azure rabbitmq k8s venafi gcp oracle dockerhub ldap github chef web salesforce postgres] | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


