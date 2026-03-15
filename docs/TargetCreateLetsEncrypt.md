# TargetCreateLetsEncrypt

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acme_challenge** | Option<**String**> |  | [optional][default to http]
**description** | Option<**String**> | Description of the object | [optional]
**dns_target_creds** | Option<**String**> | Name of existing cloud target for DNS credentials. Required when acme-challenge=dns. Supported: AWS, Azure, GCP targets | [optional]
**email** | Option<**String**> | Email address for ACME account registration | [optional]
**gcp_project** | Option<**String**> | GCP Cloud DNS: Project ID. Optional - can be derived from service account | [optional]
**hosted_zone** | Option<**String**> | AWS Route53 hosted zone ID. Required when dns-target-creds points to AWS target | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key** | Option<**String**> | The name of a key that used to encrypt the target secret value (if empty, the account default protectionKey key will be used) | [optional]
**lets_encrypt_url** | Option<**String**> |  | [optional][default to production]
**max_versions** | Option<**String**> | Set the maximum number of versions, limited by the account settings defaults. | [optional]
**name** | **String** | Target name | 
**resource_group** | Option<**String**> | Azure resource group name. Required when dns-target-creds points to Azure target | [optional]
**timeout** | Option<**String**> |  | [optional][default to 5m]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


