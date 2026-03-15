# GatewayCreateProducerLdap

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**provider_type** | Option<**String**> |  | [optional]
**bind_dn** | Option<**String**> | Bind DN | [optional]
**bind_dn_password** | Option<**String**> | Bind DN Password | [optional]
**custom_username_template** | Option<**String**> | Customize how temporary usernames are generated using go template | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**external_username** | Option<**String**> | Externally provided username [true/false] | [optional][default to false]
**fixed_user_claim_keyname** | Option<**String**> | For externally provided users, denotes the key-name of IdP claim to extract the username from (relevant only for external-username=true) | [optional][default to ext_username]
**group_dn** | Option<**String**> | Group DN which the temporary user should be added | [optional]
**host_provider** | Option<**String**> | Host provider type [explicit/target], Default Host provider is explicit, Relevant only for Secure Remote Access of ssh cert issuer, ldap rotated secret and ldap dynamic secret | [optional]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**ldap_ca_cert** | Option<**String**> | CA Certificate File Content | [optional]
**ldap_url** | Option<**String**> | LDAP Server URL | [optional]
**name** | **String** | Dynamic secret name | 
**password_length** | Option<**String**> | The length of the password to be generated | [optional]
**producer_encryption_key_name** | Option<**String**> | Dynamic producer encryption key | [optional]
**secure_access_bastion_issuer** | Option<**String**> | Deprecated. use secure-access-certificate-issuer | [optional]
**secure_access_certificate_issuer** | Option<**String**> | Path to the SSH Certificate Issuer for your Akeyless Secure Access | [optional]
**secure_access_delay** | Option<**i64**> | The delay duration, in seconds, to wait after generating just-in-time credentials. Accepted range: 0-120 seconds | [optional]
**secure_access_enable** | Option<**String**> | Enable/Disable secure remote access [true/false] | [optional]
**secure_access_host** | Option<**Vec<String>**> | Target servers for connections (In case of Linked Target association, host(s) will inherit Linked Target hosts - Relevant only for Dynamic Secrets/producers) | [optional]
**secure_access_rd_gateway_server** | Option<**String**> | RD Gateway server | [optional]
**secure_access_rdp_domain** | Option<**String**> | Required when the Dynamic Secret is used for a domain user | [optional]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**target** | Option<**Vec<String>**> | A list of linked targets to be associated, Relevant only for Secure Remote Access for ssh cert issuer, ldap rotated secret and ldap dynamic secret, To specify multiple targets use argument multiple times | [optional]
**target_name** | Option<**String**> | Target name | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**token_expiration** | Option<**String**> | Token expiration | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**user_attribute** | Option<**String**> | User Attribute | [optional]
**user_dn** | Option<**String**> | User DN | [optional]
**user_ttl** | Option<**String**> | User TTL | [optional][default to 60m]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


