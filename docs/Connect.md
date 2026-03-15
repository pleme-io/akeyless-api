# Connect

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**helper** | Option<[**serde_json::Value**](.md)> |  | [optional]
**rc_file_override** | Option<**String**> | used to override .akeyless-connect.rc in tests | [optional]
**bastion_ctrl_path** | Option<**String**> | Deprecated. use bastion-ctrl-path | [optional]
**bastion_ctrl_port** | Option<**String**> | Deprecated. use sra-ctrl-port | [optional]
**bastion_ctrl_proto** | Option<**String**> | Deprecated. use sra-ctrl-proto | [optional]
**bastion_ctrl_subdomain** | Option<**String**> | Deprecated. use sra-ctrl-subdomain | [optional]
**cert_issuer_name** | Option<**String**> | The Akeyless certificate issuer name | [optional]
**gateway_url** | Option<**String**> | The Gateway URL (configuration management) address, e.g. http://localhost:8000 | [optional]
**identity_file** | Option<**String**> | The file from which the identity (private key) for public key authentication is read | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**justification** | Option<**String**> |  | [optional]
**name** | Option<**String**> | The Secret name (for database and AWS producers - producer name) | [optional]
**sra_ctrl_path** | Option<**String**> | The Bastion API path | [optional]
**sra_ctrl_port** | Option<**String**> | The Bastion API Port | [optional][default to 9900]
**sra_ctrl_proto** | Option<**String**> | The SRA API protocol | [optional][default to http]
**sra_ctrl_subdomain** | Option<**String**> | The SRA API prefix | [optional]
**ssh_command** | Option<**String**> | Path to SSH executable. e.g. /usr/bin/ssh | [optional]
**ssh_extra_args** | Option<**String**> | Additional SSH arguments (except -i) | [optional]
**ssh_legacy_signing_alg** | Option<**bool**> | Set this option to output legacy ('ssh-rsa-cert-v01@openssh.com') signing algorithm name in the ssh certificate. | [optional][default to false]
**target** | Option<**String**> | The target | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**use_ssh_agent** | Option<**bool**> | Deprecated | [optional]
**via_bastion** | Option<**String**> | Deprecated. Use via-sra | [optional]
**via_sra** | Option<**String**> | The jump box server | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


