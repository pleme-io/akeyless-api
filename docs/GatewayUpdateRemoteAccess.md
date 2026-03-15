# GatewayUpdateRemoteAccess

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allowed_ssh_url** | Option<**String**> | Specify a valid SSH-URL to tunnel to SSH session | [optional][default to use-existing]
**allowed_urls** | Option<**String**> | List of valid URLs to redirect from the Portal back to the remote access server (in a comma-delimited list) | [optional][default to use-existing]
**default_session_ttl_minutes** | Option<**String**> | Default session TTL in minutes | [optional][default to use-existing]
**hide_session_recording** | Option<**String**> | Specifies whether to show/hide if the session is currently recorded [true/false] | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**kexalgs** | Option<**String**> | Decide which algorithm will be used as part of the SSH initial hand-shake process | [optional][default to use-existing]
**keyboard_layout** | Option<**String**> | Enable support for additional keyboard layouts | [optional][default to use-existing]
**legacy_ssh_algorithm** | Option<**String**> | Signs SSH certificates using legacy ssh-rsa-cert-01@openssh.com signing algorithm [true/false] | [optional]
**rdp_target_configuration** | Option<**String**> | Specify the usernameSubClaim that exists inside the IDP JWT, e.g. email | [optional][default to use-existing]
**ssh_target_configuration** | Option<**String**> | Specify the usernameSubClaim that exists inside the IDP JWT, e.g. email | [optional][default to use-existing]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


