# SignPkiCertWithClassicKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**common_name** | Option<**String**> | The common name to be included in the PKI certificate | [optional]
**country** | Option<**String**> | A comma-separated list of the country that will be set in the issued certificate | [optional]
**display_id** | **String** | The name of the key to use in the sign PKI Cert process | 
**dns_names** | Option<**String**> | DNS Names to be included in the PKI certificate (in a comma-delimited list) | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key_usage** | Option<**String**> | key-usage | [optional][default to DigitalSignature,KeyAgreement,KeyEncipherment]
**locality** | Option<**String**> | A comma-separated list of the locality that will be set in the issued certificate | [optional]
**organizational_units** | Option<**String**> | A comma-separated list of organizational units (OU) that will be set in the issued certificate | [optional]
**organizations** | Option<**String**> | A comma-separated list of organizations (O) that will be set in the issued certificate | [optional]
**postal_code** | Option<**String**> | A comma-separated list of the postal code that will be set in the issued certificate | [optional]
**province** | Option<**String**> | A comma-separated list of the province that will be set in the issued certificate | [optional]
**public_key_pem_data** | Option<**String**> | PublicKey using for signing in a PEM format. | [optional]
**signing_method** | **String** | SigningMethod | 
**street_address** | Option<**String**> | A comma-separated list of the street address that will be set in the issued certificate | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**ttl** | **i64** | he requested Time To Live for the certificate, in seconds | 
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**uri_sans** | Option<**String**> | The URI Subject Alternative Names to be included in the PKI certificate (in a comma-delimited list) | [optional]
**version** | **i32** | classic key version | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


