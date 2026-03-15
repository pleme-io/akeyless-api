# DynamicSecretCreateVenafi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**admin_rotation_interval_days** | Option<**i64**> | Admin credentials rotation interval (days) | [optional][default to 0]
**allow_subdomains** | Option<**bool**> | Allow subdomains | [optional]
**allowed_domains** | Option<**Vec<String>**> | Allowed domains | [optional]
**auto_generated_folder** | Option<**String**> | Auto generated folder | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Description of the object | [optional]
**enable_admin_rotation** | Option<**bool**> | Automatic admin credentials rotation | [optional][default to false]
**item_custom_fields** | Option<**std::collections::HashMap<String, String>**> | Additional custom fields to associate with the item | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**name** | **String** | Dynamic secret name | 
**producer_encryption_key_name** | Option<**String**> | Dynamic producer encryption key | [optional]
**root_first_in_chain** | Option<**bool**> | Root first in chain | [optional]
**sign_using_akeyless_pki** | Option<**bool**> | Use Akeyless PKI issuer or Venafi issuer | [optional]
**signer_key_name** | Option<**String**> | Signer key name | [optional]
**store_private_key** | Option<**bool**> | Store private key | [optional]
**tags** | Option<**Vec<String>**> | Add tags attached to this object | [optional]
**target_name** | Option<**String**> | Target name | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**user_ttl** | Option<**String**> | User TTL in time.Duration format (2160h / 129600m / etc...). When using sign-using-akeyless-pki certificates created will have this validity period, otherwise the user-ttl is taken from the Validity Period field of the Zone's' Issuing Template. When using cert-manager it is advised to have a TTL of above 60 days (1440h). For more information - https://cert-manager.io/docs/usage/certificate/ | [optional][default to 2160h]
**venafi_access_token** | Option<**String**> | Venafi Access Token to use to access the TPP environment (Relevant when using TPP) | [optional]
**venafi_api_key** | Option<**String**> | Venafi API key | [optional]
**venafi_baseurl** | Option<**String**> | Venafi Baseurl | [optional]
**venafi_client_id** | Option<**String**> | Venafi Client ID that was used when the access token was generated | [optional][default to akeyless]
**venafi_refresh_token** | Option<**String**> | Venafi Refresh Token to use when the Access Token is expired (Relevant when using TPP) | [optional]
**venafi_use_tpp** | Option<**bool**> | Venafi using TPP | [optional]
**venafi_zone** | Option<**String**> | Venafi Zone | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


