# TargetUpdateDb

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**azure_client_id** | Option<**String**> | (Optional) Client id (relevant for \"cloud-service-provider\" only) | [optional]
**azure_client_secret** | Option<**String**> | (Optional) Client secret (relevant for \"cloud-service-provider\" only) | [optional]
**azure_tenant_id** | Option<**String**> | (Optional) Tenant id (relevant for \"cloud-service-provider\" only) | [optional]
**cloud_service_provider** | Option<**String**> | (Optional) Cloud service provider (currently only supports Azure) | [optional]
**cluster_mode** | Option<**bool**> | Cluster Mode | [optional]
**comment** | Option<**String**> | Deprecated - use description | [optional]
**connection_type** | **String** | Type of connection to mssql database [credentials/cloud-identity/wallet/parent-target] | [default to credentials]
**db_name** | Option<**String**> |  | [optional]
**db_server_certificates** | Option<**String**> | (Optional) DB server certificates | [optional]
**db_server_name** | Option<**String**> | (Optional) Server name for certificate verification | [optional]
**db_type** | **String** |  | 
**description** | Option<**String**> | Description of the object | [optional]
**host** | Option<**String**> |  | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**keep_prev_version** | Option<**String**> | Whether to keep previous version [true/false]. If not set, use default according to account settings | [optional]
**key** | Option<**String**> | The name of a key that used to encrypt the target secret value (if empty, the account default protectionKey key will be used) | [optional]
**max_versions** | Option<**String**> | Set the maximum number of versions, limited by the account settings defaults. | [optional]
**mongodb_atlas** | Option<**bool**> |  | [optional]
**mongodb_atlas_api_private_key** | Option<**String**> | MongoDB Atlas private key | [optional]
**mongodb_atlas_api_public_key** | Option<**String**> | MongoDB Atlas public key | [optional]
**mongodb_atlas_project_id** | Option<**String**> | MongoDB Atlas project ID | [optional]
**mongodb_default_auth_db** | Option<**String**> | MongoDB server default authentication database | [optional]
**mongodb_uri_options** | Option<**String**> | MongoDB server URI options | [optional]
**name** | **String** | Target name | 
**new_name** | Option<**String**> | New target name | [optional]
**oracle_service_name** | Option<**String**> | Oracle db service name | [optional]
**oracle_wallet_login_type** | Option<**String**> | Oracle Wallet login type (password/mtls) | [optional]
**oracle_wallet_p12_file_data** | Option<**String**> | Oracle wallet p12 file data in base64 | [optional]
**oracle_wallet_sso_file_data** | Option<**String**> | Oracle wallet sso file data in base64 | [optional]
**parent_target_name** | Option<**String**> | Name of the parent target, relevant only when connection-type is parent-target | [optional]
**port** | Option<**String**> |  | [optional]
**pwd** | Option<**String**> |  | [optional]
**snowflake_account** | Option<**String**> |  | [optional]
**snowflake_api_private_key** | Option<**String**> | RSA Private key (base64 encoded) | [optional]
**snowflake_api_private_key_password** | Option<**String**> | The Private key passphrase | [optional]
**ssl** | Option<**bool**> | Enable/Disable SSL [true/false] | [optional][default to false]
**ssl_certificate** | Option<**String**> | SSL connection certificate | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**user_name** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


