# DynamicSecretUpdateRdp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_user_extend_session** | Option<**i64**> | AllowUserExtendSession | [optional]
**custom_username_template** | Option<**String**> | Customize how temporary usernames are generated using go template | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**fixed_user_claim_keyname** | Option<**String**> | For externally provided users, denotes the key-name of IdP claim to extract the username from (relevant only for fixed-user-only=true) | [optional][default to ext_username]
**fixed_user_only** | Option<**String**> | Allow access using externally (IdP) provided username [true/false] | [optional][default to false]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Dynamic secret name | 
**new_name** | Option<**String**> | Dynamic secret name | [optional]
**password_length** | Option<**String**> | The length of the password to be generated | [optional]
**producer_encryption_key_name** | Option<**String**> | Dynamic producer encryption key | [optional]
**rdp_admin_name** | Option<**String**> | RDP Admin Name | [optional]
**rdp_admin_pwd** | Option<**String**> | RDP Admin password | [optional]
**rdp_host_name** | Option<**String**> | Hostname | [optional]
**rdp_host_port** | Option<**String**> | Port | [optional][default to 22]
**rdp_user_groups** | Option<**String**> | Groups | [optional]
**secure_access_allow_external_user** | Option<**bool**> | Allow providing external user for a domain users | [optional][default to false]
**secure_access_bastion_issuer** | Option<**String**> | Deprecated. use secure-access-certificate-issuer | [optional]
**secure_access_certificate_issuer** | Option<**String**> | Path to the SSH Certificate Issuer for your Akeyless Secure Access | [optional]
**secure_access_delay** | Option<**i64**> | The delay duration, in seconds, to wait after generating just-in-time credentials. Accepted range: 0-120 seconds | [optional]
**secure_access_enable** | Option<**String**> | Enable/Disable secure remote access [true/false] | [optional]
**secure_access_host** | Option<**Vec<String>**> | Target servers for connections (In case of Linked Target association, host(s) will inherit Linked Target hosts - Relevant only for Dynamic Secrets/producers) | [optional]
**secure_access_rd_gateway_server** | Option<**String**> | RD Gateway server | [optional]
**secure_access_rdp_domain** | Option<**String**> | Required when the Dynamic Secret is used for a domain user | [optional]
**secure_access_rdp_user** | Option<**String**> | Override the RDP Domain username | [optional]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**target_name** | Option<**String**> | Target name | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**user_ttl** | Option<**String**> | User TTL | [optional][default to 60m]
**warn_user_before_expiration** | Option<**i64**> | WarnBeforeUserExpiration | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


