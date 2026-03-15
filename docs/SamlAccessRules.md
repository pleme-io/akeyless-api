# SamlAccessRules

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allowed_redirect_uris** | Option<**Vec<String>**> | Allowed redirect URIs after the authentication | [optional]
**bound_attributes** | Option<[**Vec<models::SamlAttribute>**](SAMLAttribute.md)> | The attributes that login is restricted to. | [optional]
**idp_metadata_url** | Option<**String**> | IDP metadata url | [optional]
**idp_metadata_xml** | Option<**String**> | IDP metadata XML | [optional]
**unique_identifier** | Option<**String**> | A unique identifier to distinguish different users | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


