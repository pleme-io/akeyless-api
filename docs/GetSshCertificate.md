# GetSshCertificate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cert_issuer_name** | **String** | The name of the SSH certificate issuer | 
**cert_username** | **String** | The username to sign in the SSH certificate | [default to -]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**legacy_signing_alg_name** | Option<**bool**> | Set this option to output legacy ('ssh-rsa-cert-v01@openssh.com') signing algorithm name in the certificate. | [optional][default to false]
**public_key_data** | Option<**String**> | SSH public key file contents. If this option is used, the certificate will be printed to stdout | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**ttl** | Option<**i64**> | Updated certificate lifetime in seconds (must be less than the Certificate Issuer default TTL) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


