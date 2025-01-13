# \IndexerStatsApi

All URIs are relative to *http://localhost:9696*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v1_indexerstats_get**](IndexerStatsApi.md#api_v1_indexerstats_get) | **GET** /api/v1/indexerstats | 



## api_v1_indexerstats_get

> models::IndexerStatsResource api_v1_indexerstats_get(start_date, end_date, indexers, protocols, tags)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | Option<**String**> |  |  |
**end_date** | Option<**String**> |  |  |
**indexers** | Option<**String**> |  |  |
**protocols** | Option<**String**> |  |  |
**tags** | Option<**String**> |  |  |

### Return type

[**models::IndexerStatsResource**](IndexerStatsResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

