# DbTargetDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cloud_service_provider** | Option<**String**> |  | [optional]
**cluster_mode** | Option<**bool**> |  | [optional]
**connection_type** | Option<**String**> |  | [optional]
**db_client_id** | Option<**String**> |  | [optional]
**db_client_secret** | Option<**String**> |  | [optional]
**db_host_name** | Option<**String**> |  | [optional]
**db_name** | Option<**String**> |  | [optional]
**db_port** | Option<**String**> |  | [optional]
**db_private_key** | Option<**String**> | (Optional) Private Key in PEM format | [optional]
**db_private_key_passphrase** | Option<**String**> |  | [optional]
**db_pwd** | Option<**String**> |  | [optional]
**db_server_certificates** | Option<**String**> | (Optional) DBServerCertificates defines the set of root certificate authorities that clients use when verifying server certificates. If DBServerCertificates is empty, TLS uses the host's root CA set. | [optional]
**db_server_name** | Option<**String**> | (Optional) ServerName is used to verify the hostname on the returned certificates unless InsecureSkipVerify is given. It is also included in the client's handshake to support virtual hosting unless it is an IP address. | [optional]
**db_tenant_id** | Option<**String**> |  | [optional]
**db_user_name** | Option<**String**> |  | [optional]
**oracle_wallet_details** | Option<[**models::WalletDetails**](WalletDetails.md)> |  | [optional]
**sf_account** | Option<**String**> |  | [optional]
**ssl_connection_certificate** | Option<**String**> | (Optional) SSLConnectionCertificate defines the certificate for SSL connection. Must be base64 certificate loaded by UI using file loader field | [optional]
**ssl_connection_mode** | Option<**bool**> | (Optional) SSLConnectionMode defines if SSL mode will be used to connect to DB | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


