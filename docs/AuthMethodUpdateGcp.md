# AuthMethodUpdateGcp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_expires** | Option<**i64**> | Access expiration date in Unix timestamp (select 0 for access without expiry date) | [optional][default to 0]
**allowed_client_type** | Option<**Vec<String>**> | limit the auth method usage for specific client types [cli,ui,gateway-admin,sdk,mobile,extension] | [optional]
**audience** | **String** | The audience to verify in the JWT received by the client | [default to akeyless.io]
**audit_logs_claims** | Option<**Vec<String>**> | Subclaims to include in audit logs, e.g \"--audit-logs-claims email --audit-logs-claims username\" | [optional]
**bound_ips** | Option<**Vec<String>**> | A CIDR whitelist with the IPs that the access is restricted to | [optional]
**bound_labels** | Option<**Vec<String>**> | A comma-separated list of GCP labels formatted as \"key:value\" strings that must be set on authorized GCE instances. TODO: Because GCP labels are not currently ACL'd .... | [optional]
**bound_projects** | Option<**Vec<String>**> | === Human and Machine authentication section === Array of GCP project IDs. Only entities belonging to any of the provided projects can authenticate. | [optional]
**bound_regions** | Option<**Vec<String>**> | List of regions that a GCE instance must belong to in order to be authenticated. TODO: If bound_instance_groups is provided, it is assumed to be a regional group and the group must belong to this region. If bound_zones are provided, this attribute is ignored. | [optional]
**bound_service_accounts** | Option<**Vec<String>**> | List of service accounts the service account must be part of in order to be authenticated. | [optional]
**bound_zones** | Option<**Vec<String>**> | === Machine authentication section === List of zones that a GCE instance must belong to in order to be authenticated. TODO: If bound_instance_groups is provided, it is assumed to be a zonal group and the group must belong to this zone. | [optional]
**delete_protection** | Option<**String**> | Protection from accidental deletion of this object [true/false] | [optional]
**description** | Option<**String**> | Auth Method description | [optional]
**expiration_event_in** | Option<**Vec<String>**> | How many days before the expiration of the auth method would you like to be notified. | [optional]
**force_sub_claims** | Option<**bool**> | if true: enforce role-association must include sub claims | [optional]
**gw_bound_ips** | Option<**Vec<String>**> | A CIDR whitelist with the GW IPs that the access is restricted to | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**jwt_ttl** | Option<**i64**> | Jwt TTL | [optional][default to 0]
**name** | **String** | Auth Method name | 
**new_name** | Option<**String**> | Auth Method new name | [optional]
**product_type** | Option<**Vec<String>**> | Choose the relevant product type for the auth method [sm, sra, pm, dp, ca] | [optional]
**service_account_creds_data** | Option<**String**> | ServiceAccount credentials data instead of giving a file path, base64 encoded | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**r#type** | **String** | Type of the GCP Access Rules | 
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]
**unique_identifier** | Option<**String**> | A unique identifier (ID) value which is a \"sub claim\" name that contains details uniquely identifying that resource. This \"sub claim\" is used to distinguish between different identities. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


