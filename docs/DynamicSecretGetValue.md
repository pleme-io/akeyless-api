# DynamicSecretGetValue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**args** | Option<**Vec<String>**> | Optional arguments as key=value pairs or JSON strings, e.g - \\\"--args=csr=base64_encoded_csr --args=common_name=bar\\\" or args='{\\\"csr\\\":\\\"base64_encoded_csr\\\"}. It is possible to combine both formats.' | [optional]
**dbname** | Option<**String**> | DBName: Optional override DB name (works only if DS allows it. only relevant for MSSQL) | [optional]
**host** | Option<**String**> | Host | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Dynamic secret name | 
**target** | Option<**String**> | Target Name | [optional]
**timeout** | Option<**i64**> | Timeout in seconds | [optional][default to 15]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


