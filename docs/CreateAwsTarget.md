# CreateAwsTarget

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_key** | **String** | AWS secret access key | 
**access_key_id** | **String** | AWS access key ID | 
**comment** | Option<**String**> | Deprecated - use description | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**generate_external_id** | Option<**bool**> | A unique auto-generated value used in your AWS account when configuring your AWS IAM role to securely delegate access to Akeyless. Relevant only when using GW cloud ID | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key** | Option<**String**> | The name of a key that used to encrypt the target secret value (if empty, the account default protectionKey key will be used) | [optional]
**max_versions** | Option<**String**> | Set the maximum number of versions, limited by the account settings defaults. | [optional]
**name** | **String** | Target name | 
**region** | Option<**String**> | AWS region | [optional][default to us-east-2]
**role_arn** | Option<**String**> | AWS IAM role identifier that Gateway will assume in your AWS account, relevant only when using external ID | [optional]
**session_token** | Option<**String**> | Required only for temporary security credentials retrieved using STS | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**use_gw_cloud_identity** | Option<**bool**> | Use the GW's Cloud IAM | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


