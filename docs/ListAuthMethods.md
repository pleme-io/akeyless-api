# ListAuthMethods

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filter by auth method name or part of it | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**pagination_token** | Option<**String**> | Next page reference | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**r#type** | Option<**Vec<String>**> | The Auth method types list of the requested method. In case it is empty, all types of auth methods will be returned. options: [api_key, azure_ad, oauth2/jwt, saml2, ldap, aws_iam, oidc, universal_identity, gcp, k8s, cert] | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


