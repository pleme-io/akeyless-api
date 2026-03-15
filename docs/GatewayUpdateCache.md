# GatewayUpdateCache

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**backup_interval** | Option<**String**> | Secure backup interval in minutes. To ensure service continuity in case of power cycle and network outage secrets will be backed up periodically per backup interval | [optional][default to 1]
**enable_cache** | Option<**String**> | Enable cache [true/false] | [optional]
**enable_proactive** | Option<**String**> | Enable proactive caching [true/false] | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**minimum_fetch_interval** | Option<**String**> | When using Cache or/and Proactive Cache, additional secrets will be fetched upon requesting a secret, based on the requestor's access policy. Define minimum fetching interval to avoid over fetching in a given time frame | [optional][default to 5]
**stale_timeout** | Option<**String**> | Stale timeout in minutes, cache entries which are not accessed within timeout will be removed from cache | [optional][default to 60]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


