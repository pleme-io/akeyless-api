# SshCertificateIssueDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allowed_domains** | Option<**Vec<String>**> | Relevant for host certificate | [optional]
**allowed_user_key_lengths** | Option<**std::collections::HashMap<String, i64>**> |  | [optional]
**allowed_users** | Option<**Vec<String>**> | Relevant for user certificate | [optional]
**cert_type** | Option<**i32**> |  | [optional]
**critical_options** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**extensions** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**externally_provided_user_sub_claim_key** | Option<**String**> | ExternallyProvidedUserSubClaimKey is the claim key name where the user name should be taken from | [optional]
**is_externally_provided_user** | Option<**bool**> | IsExternallyProvidedUser is true if allow users should be taken from claims and not from AllowedUsers | [optional]
**principals** | Option<**Vec<String>**> |  | [optional]
**static_key_id** | Option<**String**> | In case it is empty, the key ID will be combination of user identifiers and a random string | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


