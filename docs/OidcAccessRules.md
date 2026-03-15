# OidcAccessRules

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allowed_redirect_uris** | Option<**Vec<String>**> | Allowed redirect URIs after the authentication | [optional]
**audience** | Option<**String**> | Audience claim to be used as part of the authentication flow. In case set, it must match the one configured on the Identity Provider's Application | [optional]
**bound_claims** | Option<[**Vec<models::OidcCustomClaim>**](OIDCCustomClaim.md)> | The claims that login is restricted to. | [optional]
**client_id** | Option<**String**> | Client ID | [optional]
**client_secret** | Option<**String**> | Client Secret | [optional]
**is_internal** | Option<**bool**> | IsInternal indicates whether this is an internal Auth Method where the client has no control over it, or it was created by the client e.g - Sign In with Google will create an OIDC Auth Method with IsInternal=true | [optional]
**issuer** | Option<**String**> | Issuer URL | [optional]
**required_scopes** | Option<**Vec<String>**> | A list of required scopes to request from the oidc provider, and to check on the token | [optional]
**required_scopes_prefix** | Option<**String**> | A prefix to add to the required scopes (for example, azures' Application ID URI) | [optional]
**unique_identifier** | Option<**String**> | A unique identifier to distinguish different users | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


