# UpdateSshCertIssuer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider_type** | Option<**String**> |  | [optional]
**add_tag** | Option<**Vec<String>**> | List of the new tags that will be attached to this item | [optional]
**allowed_users** | **String** | Users allowed to fetch the certificate, e.g root,ubuntu | [default to -]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**extensions** | Option<**std::collections::HashMap<String, String>**> | Signed certificates with extensions, e.g permit-port-forwarding=\\\"\\\" | [optional]
**external_username** | Option<**String**> | Externally provided username [true/false] | [optional][default to false]
**fixed_user_claim_keyname** | Option<**String**> | For externally provided users, denotes the key-name of IdP claim to extract the username from (relevant only for external-username=true) | [optional]
**host_provider** | Option<**String**> | Host provider type [explicit/target], Default Host provider is explicit, Relevant only for Secure Remote Access of ssh cert issuer, ldap rotated secret and ldap dynamic secret | [optional]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**metadata** | Option<**String**> | Deprecated - use description | [optional]
**name** | **String** | SSH certificate issuer name | 
**new_name** | Option<**String**> | New item name | [optional]
**principals** | Option<**String**> | Signed certificates with principal, e.g example_role1,example_role2 | [optional]
**rm_tag** | Option<**Vec<String>**> | List of the existent tags that will be removed from this item | [optional]
**secure_access_api** | Option<**String**> | Secure Access SSH control API endpoint. E.g. https://my.sra-server:9900 | [optional]
**secure_access_bastion_api** | Option<**String**> | Deprecated. use secure-access-api | [optional]
**secure_access_bastion_ssh** | Option<**String**> | Deprecated. use secure-access-ssh | [optional]
**secure_access_enable** | Option<**String**> | Enable/Disable secure remote access [true/false] | [optional]
**secure_access_enforce_hosts_restriction** | Option<**bool**> | Enable this flag to enforce connections only to the hosts listed in --secure-access-host | [optional]
**secure_access_gateway** | Option<**String**> |  | [optional]
**secure_access_host** | Option<**Vec<String>**> | Target servers for connections (In case of Linked Target association, host(s) will inherit Linked Target hosts - Relevant only for Dynamic Secrets/producers) | [optional]
**secure_access_ssh** | Option<**String**> | Bastion's SSH server. E.g. my.sra-server:22 | [optional]
**secure_access_ssh_creds_user** | Option<**String**> | SSH username to connect to target server, must be in 'Allowed Users' list | [optional]
**secure_access_use_internal_bastion** | Option<**bool**> | Deprecated. Use secure-access-use-internal-ssh-access | [optional]
**secure_access_use_internal_ssh_access** | Option<**bool**> | Use internal SSH Access | [optional]
**signer_key_name** | **String** | A key to sign the certificate with | 
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**ttl** | **i64** | The requested Time To Live for the certificate, in seconds | 
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


