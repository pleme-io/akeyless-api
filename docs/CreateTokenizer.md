# CreateTokenizer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alphabet** | Option<**String**> | Alphabet to use in regexp vaultless tokenization | [optional]
**decoding_template** | Option<**String**> | The Decoding output template to use in regexp vaultless tokenization | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**encoding_template** | Option<**String**> | The Encoding output template to use in regexp vaultless tokenization | [optional]
**encryption_key_name** | Option<**String**> | AES key name to use in vaultless tokenization | [optional]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**metadata** | Option<**String**> | Deprecated - use description | [optional]
**name** | **String** | Tokenizer name | 
**pattern** | Option<**String**> | Pattern to use in regexp vaultless tokenization | [optional]
**tag** | Option<**Vec<String>**> | List of the tags attached to this key | [optional]
**template_type** | **String** | Which template type this tokenizer is used for [SSN,CreditCard,USPhoneNumber,Email,Regexp] | 
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**tokenizer_type** | **String** | Tokenizer type | [default to vaultless]
**tweak_type** | Option<**String**> | The tweak type to use in vaultless tokenization [Supplied, Generated, Internal, Masking] | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


