# TargetUpdateGodaddy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_key** | **String** | Key of the api credentials to the Godaddy account | 
**customer_id** | Option<**String**> | Customer ID (ShopperId) required for renewal of imported certificates | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**imap_fqdn** | **String** | ImapFQDN of the IMAP service, FQDN or IPv4 address. Must be FQDN if the IMAP is using TLS | 
**imap_password** | **String** | ImapPassword to access the IMAP service | 
**imap_port** | Option<**String**> | ImapPort of the IMAP service | [optional][default to 993]
**imap_username** | **String** | ImapUsername to access the IMAP service | 
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**keep_prev_version** | Option<**String**> | Whether to keep previous version [true/false]. If not set, use default according to account settings | [optional]
**key** | Option<**String**> | The name of a key that used to encrypt the target secret value (if empty, the account default protectionKey key will be used) | [optional]
**max_versions** | Option<**String**> | Set the maximum number of versions, limited by the account settings defaults. | [optional]
**name** | **String** | Target name | 
**new_name** | Option<**String**> | New target name | [optional]
**secret** | **String** | Secret of the api credentials to the Godaddy account | 
**timeout** | Option<**String**> | Timeout waiting for certificate validation in Duration format (1h - 1 Hour, 20m - 20 Minutes, 33m3s - 33 Minutes and 3 Seconds), maximum 1h. | [optional][default to 5m]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


