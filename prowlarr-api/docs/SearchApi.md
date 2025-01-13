# \SearchApi

All URIs are relative to *http://localhost:9696*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v1_search_bulk_post**](SearchApi.md#api_v1_search_bulk_post) | **POST** /api/v1/search/bulk | 
[**api_v1_search_get**](SearchApi.md#api_v1_search_get) | **GET** /api/v1/search | 
[**api_v1_search_post**](SearchApi.md#api_v1_search_post) | **POST** /api/v1/search | 



## api_v1_search_bulk_post

> models::ReleaseResource api_v1_search_bulk_post(release_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**release_resource** | Option<[**Vec<models::ReleaseResource>**](ReleaseResource.md)> |  |  |

### Return type

[**models::ReleaseResource**](ReleaseResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_search_get

> Vec<models::ReleaseResource> api_v1_search_get(query, r#type, indexer_ids, categories, limit, offset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> |  |  |
**r#type** | Option<**String**> |  |  |
**indexer_ids** | Option<[**Vec<i32>**](i32.md)> |  |  |
**categories** | Option<[**Vec<i32>**](i32.md)> |  |  |
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |

### Return type

[**Vec<models::ReleaseResource>**](ReleaseResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_search_post

> models::ReleaseResource api_v1_search_post(release_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**release_resource** | Option<[**ReleaseResource**](ReleaseResource.md)> |  |  |

### Return type

[**models::ReleaseResource**](ReleaseResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

