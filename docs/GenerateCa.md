# GenerateCa

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alg** | Option<**String**> |  | [optional]
**allowed_domains** | **String** | A list of the allowed domains that clients can request to be included in the certificate (in a comma-delimited list) | 
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**extended_key_usage** | Option<**String**> | A comma sepereted list of extended key usage for the intermediate (serverauth / clientauth / codesigning) | [optional][default to serverauth,clientauth]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key_type** | Option<**String**> |  | [optional]
**max_path_len** | Option<**i64**> | The maximum number of intermediate certificates that can appear in a certification path | [optional][default to 1]
**pki_chain_name** | **String** | PKI chain name | 
**protection_key_name** | Option<**String**> | The name of a key that used to encrypt the secrets values (if empty, the account default protectionKey key will be used) | [optional]
**split_level** | Option<**i64**> | The number of fragments that the item will be split into | [optional][default to 3]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**ttl** | **String** | The maximum requested Time To Live for issued certificate by default in seconds, supported formats are s,m,h,d | 
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


