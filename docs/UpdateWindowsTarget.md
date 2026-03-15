# UpdateWindowsTarget

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**certificate** | Option<**String**> | SSL CA certificate in base64 encoding generated from a trusted Certificate Authority (CA) | [optional]
**connection_type** | Option<**String**> | Type of connection to Windows Server [credentials/parent-target] | [optional][default to credentials]
**description** | Option<**String**> | Description of the object | [optional]
**domain** | Option<**String**> | User domain name | [optional]
**hostname** | **String** | Server hostname | 
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**keep_prev_version** | Option<**String**> | Whether to keep previous version [true/false]. If not set, use default according to account settings | [optional]
**key** | Option<**String**> | The name of a key that used to encrypt the target secret value (if empty, the account default protectionKey key will be used) | [optional]
**max_versions** | Option<**String**> | Set the maximum number of versions, limited by the account settings defaults. | [optional]
**name** | **String** | Target name | 
**new_name** | Option<**String**> | New target name | [optional]
**parent_target_name** | Option<**String**> | Name of the parent target, relevant only when connection-type is parent-target | [optional]
**password** | **String** | Privileged user password | [default to dummy_value]
**port** | Option<**String**> | Server WinRM port | [optional][default to 5986]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**update_version** | Option<**bool**> | Deprecated | [optional]
**use_tls** | Option<**String**> | Enable/Disable TLS for WinRM over HTTPS [true/false] | [optional][default to true]
**username** | **String** | Privileged username | [default to dummy_value]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


