# CreatePasskey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accessibility** | Option<**String**> | for personal password manager | [optional][default to regular]
**alg** | **String** | Passkey type; options: [EC256, EC384, EC512] | [default to EC256]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | ClassicKey name | 
**origin_url** | Option<**Vec<String>**> | Originating websites for this passkey | [optional]
**protection_key_name** | Option<**String**> | The name of a key that used to encrypt the secret value (if empty, the account default protectionKey key will be used) | [optional]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**username** | Option<**String**> | For Password Management use | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


