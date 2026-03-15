# GatewayUpdateRemoteAccessRdpRecordings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aws_storage_access_key_id** | Option<**String**> | AWS access key id. For more information refer to https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_access-keys.html | [optional]
**aws_storage_bucket_name** | Option<**String**> | The AWS bucket name. For more information refer to https://docs.aws.amazon.com/s3/ | [optional]
**aws_storage_bucket_prefix** | Option<**String**> | The folder name in S3 bucket. For more information refer to https://docs.aws.amazon.com/s3/ | [optional]
**aws_storage_region** | Option<**String**> | The region where the storage is located | [optional]
**aws_storage_secret_access_key** | Option<**String**> | AWS secret access key. For more information refer to https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_access-keys.html | [optional]
**azure_storage_account_name** | Option<**String**> | Azure account name. For more information refer to https://learn.microsoft.com/en-us/azure/storage/common/storage-account-overview | [optional]
**azure_storage_client_id** | Option<**String**> | Azure client id. For more information refer to https://learn.microsoft.com/en-us/azure/storage/common/storage-account-get-info?tabs=portal | [optional]
**azure_storage_client_secret** | Option<**String**> | Azure client secret. For more information refer to https://learn.microsoft.com/en-us/azure/storage/common/storage-account-get-info?tabs=portal | [optional]
**azure_storage_container_name** | Option<**String**> | Azure container name. For more information refer to https://learn.microsoft.com/en-us/rest/api/storageservices/naming-and-referencing-containers--blobs--and-metadata | [optional]
**azure_storage_tenant_id** | Option<**String**> | Azure tenant id. For more information refer to https://learn.microsoft.com/en-us/entra/fundamentals/how-to-find-tenant | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**rdp_session_recording** | Option<**String**> | Enable recording of rdp session [true/false] | [optional]
**rdp_session_recording_compress** | Option<**bool**> | Whether to compress recording files before upload | [optional]
**rdp_session_recording_encryption_key** | Option<**String**> | If provided, this key will be used to encrypt uploaded recordings. | [optional]
**rdp_session_recording_quality** | Option<**String**> | RDP session recording quality [low/medium/high] | [optional]
**rdp_session_storage** | Option<**String**> | Rdp session recording storage destination [local/aws/azure] | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


