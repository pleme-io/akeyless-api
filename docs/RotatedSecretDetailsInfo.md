# RotatedSecretDetailsInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delete_previous_version_in_days** | Option<**i32**> |  | [optional]
**enable_custom_password_policy** | Option<**bool**> |  | [optional]
**grace_rotation** | Option<**bool**> |  | [optional]
**grace_rotation_hour** | Option<**i32**> |  | [optional]
**grace_rotation_interval** | Option<**i32**> |  | [optional]
**grace_rotation_timing** | Option<**String**> |  | [optional]
**gw_cluster_id** | Option<**i64**> |  | [optional]
**iis_apps_details** | Option<[**Vec<models::WindowsService>**](WindowsService.md)> |  | [optional]
**last_rotation_error** | Option<**String**> |  | [optional]
**managed_by_akeyless** | Option<**bool**> |  | [optional]
**max_versions** | Option<**i64**> |  | [optional]
**next_auto_rotate_type** | Option<**String**> |  | [optional]
**number_of_versions_to_save** | Option<**i32**> |  | [optional]
**public_key_remote_path** | Option<**String**> |  | [optional]
**rotation_hour** | Option<**i32**> |  | [optional]
**rotation_interval_min** | Option<**bool**> |  | [optional]
**rotation_statement** | Option<**String**> |  | [optional]
**rotator_creds_type** | Option<**String**> |  | [optional]
**rotator_status** | Option<**String**> | RotationStatus defines types of rotation Status | [optional]
**rotator_type** | Option<**String**> |  | [optional]
**same_password** | Option<**bool**> |  | [optional]
**services_details** | Option<[**Vec<models::WindowsService>**](WindowsService.md)> |  | [optional]
**timeout_seconds** | Option<**i64**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


