# OAuth2AccessRules

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**audience** | Option<**String**> | The audience in the JWT. | [optional]
**authorized_gw_cluster_name** | Option<**String**> | The gateway cluster name that is authorized to access JWKeySetURL | [optional]
**bound_claims** | Option<[**Vec<models::OAuth2CustomClaim>**](OAuth2CustomClaim.md)> | The claims that login is restricted to. | [optional]
**bound_clients_id** | Option<**Vec<String>**> | The clients ids that login is restricted to. | [optional]
**certificate** | Option<**String**> | Certificate to use when calling jwks_uri from the gateway. in PEM format | [optional]
**issuer** | Option<**String**> | Issuer URL | [optional]
**jwks_json_data** | Option<**String**> | The JSON Web Key Set (JWKS) that containing the public keys that should be used to verify any JSON Web Token (JWT) issued by the authorization server. base64 encoded string | [optional]
**jwks_uri** | Option<**String**> | The URL to the JSON Web Key Set (JWKS) that containing the public keys that should be used to verify any JSON Web Token (JWT) issued by the authorization server. | [optional]
**unique_identifier** | Option<**String**> | A unique identifier to distinguish different users | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


