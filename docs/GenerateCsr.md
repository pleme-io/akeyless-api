# GenerateCsr

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alg** | Option<**String**> |  | [optional]
**alt_names** | Option<**String**> | A comma-separated list of dns alternative names | [optional]
**certificate_type** | Option<**String**> | The certificate type to be included in the CSR certificate (ssl-client/ssl-server/certificate-signing) | [optional]
**city** | Option<**String**> | The city to be included in the CSR certificate | [optional]
**common_name** | **String** | The common name to be included in the CSR certificate | 
**country** | Option<**String**> | The country to be included in the CSR certificate | [optional]
**critical** | Option<**bool**> | Add critical to the key usage extension (will be false if not added) | [optional]
**dep** | Option<**String**> | The department to be included in the CSR certificate | [optional]
**email_addresses** | Option<**String**> | A comma-separated list of email addresses alternative names | [optional]
**export_private_key** | Option<**bool**> | The flag to indicate if the private key should be exported | [optional][default to false]
**generate_key** | Option<**bool**> | Generate a new classic key for the csr | [optional]
**hash_algorithm** | Option<**String**> | Specifies the hash algorithm used for the encryption key's operations, available options: SHA256, SHA384, SHA512 | [optional][default to SHA256]
**ip_addresses** | Option<**String**> | A comma-separated list of ip addresses alternative names | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key_type** | **String** | The type of the key to generate (classic-key/dfc) | [default to classic-key]
**name** | **String** | The key name | 
**org** | Option<**String**> | The organization to be included in the CSR certificate | [optional]
**split_level** | Option<**i64**> | The number of fragments that the item will be split into (not includes customer fragment) | [optional][default to 3]
**state** | Option<**String**> | The state to be included in the CSR certificate | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**uri_sans** | Option<**String**> | A comma-separated list of uri alternative names | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


