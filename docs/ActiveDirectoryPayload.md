# ActiveDirectoryPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active_directory_target_id** | Option<**i64**> |  | [optional]
**ai_certificate_discovery** | Option<**bool**> |  | [optional]
**auto_rotate** | Option<**bool**> |  | [optional]
**auto_rotate_interval_in_days** | Option<**i32**> |  | [optional]
**auto_rotate_rotation_hour** | Option<**i32**> |  | [optional]
**certificates_expiration_events** | Option<[**Vec<models::CertificateExpirationEvent>**](CertificateExpirationEvent.md)> |  | [optional]
**certificates_path_template** | Option<**String**> |  | [optional]
**computer_base_dn** | Option<**String**> |  | [optional]
**discover_iis_apps** | Option<**bool**> |  | [optional]
**discover_local_users** | Option<**bool**> | Deprecated | [optional]
**discover_services** | Option<**bool**> |  | [optional]
**discovery_types** | Option<**Vec<String>**> |  | [optional]
**domain_name** | Option<**String**> |  | [optional]
**domain_server_targets_path_template** | Option<**String**> |  | [optional]
**domain_users_rotated_secrets_path_template** | Option<**String**> |  | [optional]
**enable_rdp_sra** | Option<**bool**> |  | [optional]
**local_users_ignore_list** | Option<**std::collections::HashMap<String, bool>**> |  | [optional]
**local_users_rotated_secrets_path_template** | Option<**String**> |  | [optional]
**os_filter** | Option<**String**> |  | [optional]
**ssh_port** | Option<**String**> |  | [optional]
**target_format** | Option<**String**> |  | [optional]
**targets_type** | Option<**String**> |  | [optional]
**user_base_dn** | Option<**String**> |  | [optional]
**user_groups** | Option<**Vec<String>**> |  | [optional]
**winrm_over_http** | Option<**bool**> |  | [optional]
**winrm_port** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


