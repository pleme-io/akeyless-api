# DeriveKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accessibility** | Option<**String**> | for personal password manager | [optional][default to regular]
**alg** | **String** | Kdf algorithm | [default to pbkdf2]
**hash_function** | Option<**String**> | HashFunction the hash function to use (relevant for pbkdf2) | [optional][default to sha256]
**iter** | **i64** | IterationCount the number of iterations | 
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**key_len** | **i64** | KeyLength the byte length of the generated key | 
**mem** | Option<**i64**> | MemorySizeInKb the memory paramter in kb (relevant for argon2id) | [optional][default to 16384]
**name** | **String** | Static Secret full name | 
**parallelism** | Option<**i64**> | Parallelism the number of threads to use (relevant for argon2id) | [optional][default to 1]
**salt** | Option<**String**> | Salt Base64 encoded salt value. If not provided, the salt will be generated as part of the operation. The salt should be securely-generated random bytes, minimum 64 bits, 128 bits is recommended | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


