# CertificateDiscovery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**debug** | Option<**bool**> | Debug mode | [optional][default to false]
**expiration_event_in** | Option<**Vec<String>**> | How many days before the expiration of the certificate would you like to be notified. | [optional]
**hosts** | **String** | A comma separated list of IPs, CIDR ranges, or DNS names to discovery | 
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**port_ranges** | Option<**String**> | A comma separated list of port ranges Examples: \"80,443\" or \"80,443,8080-8090\" or \"443\" | [optional][default to 443]
**protection_key** | Option<**String**> | The name of the key that protects the certificate value | [optional]
**target_location** | **String** | The folder where the results will be saved | 
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


