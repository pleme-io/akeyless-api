# CreateLdapTarget

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bind_dn** | **String** | Bind DN | 
**bind_dn_password** | **String** | Bind DN Password | 
**comment** | Option<**String**> | Deprecated - use description | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key** | Option<**String**> | The name of a key that used to encrypt the target secret value (if empty, the account default protectionKey key will be used) | [optional]
**ldap_ca_cert** | Option<**String**> | CA Certificate File Content | [optional]
**ldap_url** | **String** | LDAP Server URL | 
**max_versions** | Option<**String**> | Set the maximum number of versions, limited by the account settings defaults. | [optional]
**name** | **String** | Target name | 
**server_type** | Option<**String**> | Set Ldap server type, Options:[OpenLDAP, ActiveDirectory]. Default is OpenLDAP | [optional][default to OpenLDAP]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**token_expiration** | Option<**String**> | Token expiration | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


