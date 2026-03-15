# Item

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_date** | Option<**String**> |  | [optional]
**access_date_display** | Option<**String**> |  | [optional]
**access_request_status** | Option<**String**> |  | [optional]
**auto_rotate** | Option<**bool**> |  | [optional]
**bastion_details** | Option<[**models::BastionsList**](BastionsList.md)> |  | [optional]
**cert_issuer_signer_key_name** | Option<**String**> |  | [optional]
**certificate_issue_details** | Option<[**models::CertificateIssueInfo**](CertificateIssueInfo.md)> |  | [optional]
**certificates** | Option<**String**> |  | [optional]
**client_permissions** | Option<**Vec<String>**> |  | [optional]
**creation_date** | Option<**String**> |  | [optional]
**customer_fragment_id** | Option<**String**> |  | [optional]
**delete_protection** | Option<**bool**> |  | [optional]
**deletion_date** | Option<**String**> |  | [optional]
**display_id** | Option<**String**> |  | [optional]
**gateway_details** | Option<[**Vec<models::GatewayDetailsForItemReplyObj>**](GatewayDetailsForItemReplyObj.md)> |  | [optional]
**is_access_request_enabled** | Option<**bool**> |  | [optional]
**is_enabled** | Option<**bool**> |  | [optional]
**item_accessibility** | Option<**i64**> |  | [optional]
**item_custom_fields_details** | Option<[**Vec<models::ItemCustomFieldsDetails>**](ItemCustomFieldsDetails.md)> |  | [optional]
**item_general_info** | Option<[**models::ItemGeneralInfo**](ItemGeneralInfo.md)> |  | [optional]
**item_id** | Option<**i64**> |  | [optional]
**item_metadata** | Option<**String**> |  | [optional]
**item_name** | Option<**String**> |  | [optional]
**item_size** | Option<**i64**> |  | [optional]
**item_state** | Option<**String**> | ItemState defines the different states an Item can be in | [optional]
**item_sub_type** | Option<**String**> |  | [optional]
**item_tags** | Option<**Vec<String>**> |  | [optional]
**item_targets_assoc** | Option<[**Vec<models::ItemTargetAssociation>**](ItemTargetAssociation.md)> |  | [optional]
**item_type** | Option<**String**> |  | [optional]
**item_versions** | Option<[**Vec<models::ItemVersion>**](ItemVersion.md)> |  | [optional]
**last_rotation_date** | Option<**String**> |  | [optional]
**last_version** | Option<**i32**> |  | [optional]
**linked_details** | Option<[**models::LinkedDetails**](LinkedDetails.md)> |  | [optional]
**modification_date** | Option<**String**> |  | [optional]
**next_rotation_date** | Option<**String**> |  | [optional]
**protection_key_name** | Option<**String**> |  | [optional]
**protection_key_type** | Option<**String**> |  | [optional]
**public_value** | Option<**String**> |  | [optional]
**rotation_interval** | Option<**i64**> |  | [optional]
**shared_by** | Option<[**models::RuleAssigner**](RuleAssigner.md)> |  | [optional]
**target_versions** | Option<[**Vec<models::TargetItemVersion>**](TargetItemVersion.md)> |  | [optional]
**usc_sync_associated_items** | Option<[**Vec<models::ItemUscSyncAssociation>**](ItemUSCSyncAssociation.md)> | for USC item, hold rotated-secrets that are associated to him for rotated-secret, holds the associated USCs | [optional]
**with_customer_fragment** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


