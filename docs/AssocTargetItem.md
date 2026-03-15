# AssocTargetItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**certificate_path** | Option<**String**> | A path on the target to store the certificate pem file (relevant only for certificate provisioning) | [optional]
**chain_path** | Option<**String**> | A path on the target to store the full chain pem file (relevant only for certificate provisioning) | [optional]
**disable_previous_key_version** | Option<**bool**> | Automatically disable previous key version (required for azure targets) | [optional][default to false]
**external_key_name** | Option<**String**> | The external key name to associate with the classic key (Relevant only for Classic Key AWS/Azure/GCP targets) | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key_operations** | Option<**Vec<String>**> | A list of allowed operations for the key (required for azure targets) | [optional]
**keyring_name** | Option<**String**> | Keyring name of the GCP KMS (required for gcp targets) | [optional]
**kms_algorithm** | Option<**String**> | Algorithm of the key in GCP KMS (required for gcp targets) | [optional]
**location_id** | Option<**String**> | Location id of the GCP KMS (required for gcp targets) | [optional]
**multi_region** | Option<**String**> | Set to 'true' to create a multi-region managed key. (Relevant only for Classic Key AWS targets) | [optional][default to false]
**name** | **String** | The item to associate | 
**post_provision_command** | Option<**String**> | A custom command to run on the remote target after successful provisioning (relevant only for certificate provisioning) | [optional]
**private_key_path** | Option<**String**> | A path on the target to store the private key (relevant only for certificate provisioning) | [optional]
**project_id** | Option<**String**> | Project id of the GCP KMS (required for gcp targets) | [optional]
**protection_level** | Option<**String**> | Protection level of the key [software/hardware] (relevant for gcp targets) | [optional][default to software]
**purpose** | Option<**String**> | Purpose of the key in GCP KMS (required for gcp targets) | [optional]
**regions** | Option<**Vec<String>**> | The list of regions to create a copy of the key in (relevant for aws targets) | [optional]
**sra_association** | Option<**bool**> | Is the target to associate is for sra, relevant only for linked target association for ldap rotated secret | [optional][default to false]
**target_name** | **String** | The target to associate | 
**tenant_secret_type** | Option<**String**> | The tenant secret type [Data/SearchIndex/Analytics] (required for salesforce targets) | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**vault_name** | Option<**String**> | Name of the vault used (required for azure targets) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


