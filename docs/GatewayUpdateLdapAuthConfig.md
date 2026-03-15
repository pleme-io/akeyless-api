# GatewayUpdateLdapAuthConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_id** | Option<**String**> | The access ID of the Ldap auth method | [optional]
**bind_dn** | Option<**String**> | Bind DN | [optional]
**bind_dn_password** | Option<**String**> | Bind DN Password | [optional]
**group_attr** | Option<**String**> | Group Attr | [optional]
**group_dn** | Option<**String**> | Group Dn | [optional]
**group_filter** | Option<**String**> | Group Filter | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**ldap_ca_cert** | Option<**String**> | LDAP CA Certificate (base64 encoded) | [optional]
**ldap_enable** | Option<**String**> | Enable Ldap [true/false] | [optional]
**ldap_url** | Option<**String**> | LDAP Server URL, e.g. ldap://planetexpress.com:389 | [optional]
**signing_key_data** | Option<**String**> | The private key (base64 encoded), associated with the public key defined in the Ldap auth | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**user_attribute** | Option<**String**> | User Attribute | [optional]
**user_dn** | Option<**String**> | User DN | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


