# TargetCreateSectigo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**certificate_profile_id** | **i64** | Certificate Profile ID in Sectigo account | 
**customer_uri** | **String** | Customer Uri of the Sectigo account | 
**description** | Option<**String**> | Description of the object | [optional]
**external_requester** | **String** | External Requester - a comma separated list of emails | 
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key** | Option<**String**> | The name of a key that used to encrypt the target secret value (if empty, the account default protectionKey key will be used) | [optional]
**max_versions** | Option<**String**> | Set the maximum number of versions, limited by the account settings defaults. | [optional]
**name** | **String** | Target name | 
**organization_id** | **i64** | Organization ID in Sectigo account | 
**password** | **String** | Password of the Sectigo account user | 
**timeout** | Option<**String**> | Timeout waiting for certificate validation in Duration format (1h - 1 Hour, 20m - 20 Minutes, 33m3s - 33 Minutes and 3 Seconds), maximum 1h. | [optional][default to 5m]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**username** | **String** | Username of the Sectigo account | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


