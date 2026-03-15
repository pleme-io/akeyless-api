# ListItems

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accessibility** | Option<**String**> | for personal password manager | [optional][default to regular]
**advanced_filter** | Option<**String**> | Filter by item name/username/website or part of it | [optional]
**auto_pagination** | Option<**String**> | Retrieve all items using pagination, when disabled retrieving only first 1000 items | [optional][default to enabled]
**current_folder** | Option<**bool**> | List only items in the current folder (excludes subfolders) | [optional][default to false]
**filter** | Option<**String**> | Filter by item name or part of it | [optional]
**json** | Option<**bool**> | Set output format to JSON | [optional][default to false]
**minimal_view** | Option<**bool**> | Show only basic information of the items | [optional]
**modified_after** | Option<**i64**> | List only secrets modified after specified date (in unix time) | [optional]
**pagination_token** | Option<**String**> | Next page reference | [optional]
**path** | Option<**String**> | Path to folder | [optional]
**sra_only** | Option<**bool**> | Filter by items with SRA functionality enabled | [optional][default to false]
**sub_types** | Option<**Vec<String>**> |  | [optional]
**tag** | Option<**String**> | Filter by item tag | [optional]
**token** | Option<**String**> | Authentication token (see `/auth` and `/configure`) | [optional]
**r#type** | Option<**Vec<String>**> | The item types list of the requested items. In case it is empty, all types of items will be returned. options: [key, static-secret, dynamic-secret, classic-key] | [optional]
**uid_token** | Option<**String**> | The universal identity token, Required only for universal_identity authentication | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


