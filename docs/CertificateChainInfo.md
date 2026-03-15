# CertificateChainInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auto_renew_certificate** | Option<**bool**> |  | [optional]
**certificate_chain** | Option<[**Vec<models::CertificateInfo>**](CertificateInfo.md)> |  | [optional]
**certificate_format** | Option<**String**> |  | [optional]
**certificate_has_private_key** | Option<**bool**> |  | [optional]
**certificate_issuer_gw_cluster_id** | Option<**i64**> |  | [optional]
**certificate_issuer_gw_cluster_url** | Option<**String**> |  | [optional]
**certificate_issuer_item_id** | Option<**i64**> |  | [optional]
**certificate_issuer_name** | Option<**String**> |  | [optional]
**certificate_pem** | Option<**String**> |  | [optional]
**certificate_status** | Option<**String**> |  | [optional]
**common_name** | Option<**String**> |  | [optional]
**csr_pem** | Option<**String**> | CSRPEM contains the PEM-encoded CSR for pending certificates (HTTP-01 challenge) | [optional]
**error_message** | Option<**String**> |  | [optional]
**expiration_date** | Option<**String**> |  | [optional]
**expiration_events** | Option<[**Vec<models::CertificateExpirationEvent>**](CertificateExpirationEvent.md)> |  | [optional]
**external_ca_id** | Option<[**models::NullString**](NullString.md)> |  | [optional]
**issuance_status** | Option<**String**> |  | [optional]
**not_before** | Option<**String**> |  | [optional]
**renew_before_expiration_in_days** | Option<**i64**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


