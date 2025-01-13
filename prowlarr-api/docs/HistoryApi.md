# \HistoryApi

All URIs are relative to *http://localhost:9696*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v1_history_get**](HistoryApi.md#api_v1_history_get) | **GET** /api/v1/history | 
[**api_v1_history_indexer_get**](HistoryApi.md#api_v1_history_indexer_get) | **GET** /api/v1/history/indexer | 
[**api_v1_history_since_get**](HistoryApi.md#api_v1_history_since_get) | **GET** /api/v1/history/since | 



## api_v1_history_get

> models::HistoryResourcePagingResource api_v1_history_get(page, page_size, sort_key, sort_direction, event_type, successful, download_id, indexer_ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**page_size** | Option<**i32**> |  |  |[default to 10]
**sort_key** | Option<**String**> |  |  |
**sort_direction** | Option<[**SortDirection**](.md)> |  |  |
**event_type** | Option<[**Vec<i32>**](i32.md)> |  |  |
**successful** | Option<**bool**> |  |  |
**download_id** | Option<**String**> |  |  |
**indexer_ids** | Option<[**Vec<i32>**](i32.md)> |  |  |

### Return type

[**models::HistoryResourcePagingResource**](HistoryResourcePagingResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_history_indexer_get

> Vec<models::HistoryResource> api_v1_history_indexer_get(indexer_id, event_type, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**indexer_id** | Option<**i32**> |  |  |
**event_type** | Option<[**HistoryEventType**](.md)> |  |  |
**limit** | Option<**i32**> |  |  |

### Return type

[**Vec<models::HistoryResource>**](HistoryResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_history_since_get

> Vec<models::HistoryResource> api_v1_history_since_get(date, event_type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date** | Option<**String**> |  |  |
**event_type** | Option<[**HistoryEventType**](.md)> |  |  |

### Return type

[**Vec<models::HistoryResource>**](HistoryResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

