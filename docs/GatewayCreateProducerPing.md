# GatewayCreateProducerPing

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Dynamic secret name | 
**ping_administrative_port** | Option<**String**> | Ping Federate administrative port | [optional][default to 9999]
**ping_atm_id** | Option<**String**> | Set a specific Access Token Management (ATM) instance for the created OAuth Client by providing the ATM Id. If no explicit value is given, the default pingfederate server ATM will be set. | [optional]
**ping_authorization_port** | Option<**String**> | Ping Federate authorization port | [optional][default to 9031]
**ping_cert_subject_dn** | Option<**String**> | The subject DN of the client certificate. If no explicit value is given, the producer will create CA certificate and matched client certificate and return it as value. Used in conjunction with ping-issuer-dn (relevant for CLIENT_TLS_CERTIFICATE authentication method) | [optional]
**ping_client_authentication_type** | Option<**String**> | OAuth Client Authentication Type [CLIENT_SECRET, PRIVATE_KEY_JWT, CLIENT_TLS_CERTIFICATE] | [optional][default to CLIENT_SECRET]
**ping_enforce_replay_prevention** | Option<**String**> | Determines whether PingFederate requires a unique signed JWT from the client for each action (relevant for PRIVATE_KEY_JWT authentication method) [true/false] | [optional][default to false]
**ping_grant_types** | Option<**Vec<String>**> | List of OAuth client grant types [IMPLICIT, AUTHORIZATION_CODE, CLIENT_CREDENTIALS, TOKEN_EXCHANGE, REFRESH_TOKEN, ASSERTION_GRANTS, PASSWORD, RESOURCE_OWNER_CREDENTIALS]. If no explicit value is given, AUTHORIZATION_CODE will be selected as default. | [optional]
**ping_issuer_dn** | Option<**String**> | Issuer DN of trusted CA certificate that imported into Ping Federate server. You may select \\\"Trust Any\\\" to trust all the existing issuers in Ping Federate server. Used in conjunction with ping-cert-subject-dn (relevant for CLIENT_TLS_CERTIFICATE authentication method) | [optional]
**ping_jwks** | Option<**String**> | Base64-encoded JSON Web Key Set (JWKS). If no explicit value is given, the producer will create JWKs and matched signed JWT (Sign Algo: RS256) and return it as value (relevant for PRIVATE_KEY_JWT authentication method) | [optional]
**ping_jwks_url** | Option<**String**> | The URL of the JSON Web Key Set (JWKS). If no explicit value is given, the producer will create JWKs and matched signed JWT and return it as value (relevant for PRIVATE_KEY_JWT authentication method) | [optional]
**ping_password** | Option<**String**> | Ping Federate privileged user password | [optional]
**ping_privileged_user** | Option<**String**> | Ping Federate privileged user | [optional]
**ping_redirect_uris** | Option<**Vec<String>**> | List of URIs to which the OAuth authorization server may redirect the resource owner's user agent after authorization is obtained. At least one redirection URI is required for the AUTHORIZATION_CODE and IMPLICIT grant types. | [optional]
**ping_restricted_scopes** | Option<**Vec<String>**> | Limit the OAuth client to specific scopes list | [optional]
**ping_signing_algo** | Option<**String**> | The signing algorithm that the client must use to sign its request objects [RS256,RS384,RS512,ES256,ES384,ES512,PS256,PS384,PS512] If no explicit value is given, the client can use any of the supported signing algorithms (relevant for PRIVATE_KEY_JWT authentication method) | [optional]
**ping_url** | Option<**String**> | Ping URL | [optional]
**producer_encryption_key_name** | Option<**String**> | Dynamic producer encryption key | [optional]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**target_name** | Option<**String**> | Target name | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**user_ttl** | Option<**String**> | The time from dynamic secret creation to expiration. | [optional][default to 60m]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


