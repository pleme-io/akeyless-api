# GwUpdateRemoteAccessSessionLogsAwsS3

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_id** | Option<**String**> | AWS access id relevant for access_key auth-type | [optional]
**access_key** | Option<**String**> | AWS access key relevant for access_key auth-type | [optional]
**auth_type** | Option<**String**> | AWS auth type [access_key/cloud_id/assume_role] | [optional]
**bucket_name** | Option<**String**> | AWS S3 bucket name | [optional]
**enable** | Option<**String**> | Enable Log Forwarding [true/false] | [optional][default to true]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**log_folder** | Option<**String**> | AWS S3 destination folder for logs | [optional][default to use-existing]
**output_format** | Option<**String**> | Logs format [text/json] | [optional][default to text]
**pull_interval** | Option<**String**> | Pull interval in seconds | [optional][default to 10]
**region** | Option<**String**> | AWS region | [optional]
**role_arn** | Option<**String**> | AWS role arn relevant for assume_role auth-type | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


