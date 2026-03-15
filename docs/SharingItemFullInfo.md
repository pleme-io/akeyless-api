# SharingItemFullInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assigners** | Option<[**Vec<models::RuleAssigner>**](RuleAssigner.md)> |  | [optional]
**capabilities** | Option<**Vec<String>**> | The approved/denied capabilities in the path | [optional]
**cb** | Option<**i32**> |  | [optional]
**is_limit_access** | Option<**bool**> | flag that indicate that this rule is allowed to be access RemainingAccess of times. | [optional]
**item_id** | Option<**i64**> | The item id this rule directly refers to (when applicable) | [optional]
**name** | Option<**String**> |  | [optional]
**number_of_access_used** | Option<**i64**> |  | [optional]
**number_of_allowed_access** | Option<**i64**> |  | [optional]
**path** | Option<**String**> | The path the rule refers to | [optional]
**start_time** | Option<**i64**> |  | [optional]
**ttl** | Option<**i64**> |  | [optional]
**r#type** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


