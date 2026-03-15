# GatewayUpdateProducerAws

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_mode** | Option<**String**> |  | [optional]
**admin_rotation_interval_days** | Option<**i64**> | Admin credentials rotation interval (days) | [optional][default to 0]
**aws_access_key_id** | Option<**String**> | Access Key ID | [optional]
**aws_access_secret_key** | Option<**String**> | Secret Access Key | [optional]
**aws_external_id** | Option<**String**> | The AWS External ID associated with the AWS role (relevant only for assume_role mode) | [optional]
**aws_role_arns** | Option<**String**> | AWS Role ARNs to be used in the Assume Role operation (relevant only for assume_role mode) | [optional]
**aws_user_console_access** | Option<**bool**> | AWS User console access | [optional][default to false]
**aws_user_groups** | Option<**String**> | AWS User groups | [optional]
**aws_user_policies** | Option<**String**> | AWS User policies | [optional]
**aws_user_programmatic_access** | Option<**bool**> | Enable AWS User programmatic access | [optional][default to true]
**custom_username_template** | Option<**String**> | Customize how temporary usernames are generated using go template | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**enable_admin_rotation** | Option<**bool**> | Automatic admin credentials rotation | [optional][default to false]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Dynamic secret name | 
**new_name** | Option<**String**> | Dynamic secret name | [optional]
**password_length** | Option<**String**> | The length of the password to be generated | [optional]
**producer_encryption_key_name** | Option<**String**> | Dynamic producer encryption key | [optional]
**region** | Option<**String**> | Region | [optional][default to us-east-2]
**secure_access_aws_account_id** | Option<**String**> | The AWS account id | [optional]
**secure_access_aws_native_cli** | Option<**bool**> | The AWS native cli | [optional]
**secure_access_bastion_issuer** | Option<**String**> | Deprecated. use secure-access-certificate-issuer | [optional]
**secure_access_certificate_issuer** | Option<**String**> | Path to the SSH Certificate Issuer for your Akeyless Secure Access | [optional]
**secure_access_delay** | Option<**i64**> | The delay duration, in seconds, to wait after generating just-in-time credentials. Accepted range: 0-120 seconds | [optional]
**secure_access_enable** | Option<**String**> | Enable/Disable secure remote access [true/false] | [optional]
**secure_access_web** | Option<**bool**> | Enable Web Secure Remote Access | [optional][default to true]
**secure_access_web_browsing** | Option<**bool**> | Secure browser via Akeyless's Secure Remote Access (SRA) | [optional][default to false]
**secure_access_web_proxy** | Option<**bool**> | Web-Proxy via Akeyless's Secure Remote Access (SRA) | [optional][default to false]
**session_tags** | Option<**String**> | String of Key value session tags comma separated, relevant only for Assumed Role | [optional]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**target_name** | Option<**String**> | Target name | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**transitive_tag_keys** | Option<**String**> | String of transitive tag keys space separated, relevant only for Assumed Role | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**user_ttl** | Option<**String**> | User TTL | [optional][default to 60m]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


