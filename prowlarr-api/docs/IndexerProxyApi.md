# \IndexerProxyApi

All URIs are relative to *http://localhost:9696*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v1_indexerproxy_action_name_post**](IndexerProxyApi.md#api_v1_indexerproxy_action_name_post) | **POST** /api/v1/indexerproxy/action/{name} | 
[**api_v1_indexerproxy_get**](IndexerProxyApi.md#api_v1_indexerproxy_get) | **GET** /api/v1/indexerproxy | 
[**api_v1_indexerproxy_id_delete**](IndexerProxyApi.md#api_v1_indexerproxy_id_delete) | **DELETE** /api/v1/indexerproxy/{id} | 
[**api_v1_indexerproxy_id_get**](IndexerProxyApi.md#api_v1_indexerproxy_id_get) | **GET** /api/v1/indexerproxy/{id} | 
[**api_v1_indexerproxy_id_put**](IndexerProxyApi.md#api_v1_indexerproxy_id_put) | **PUT** /api/v1/indexerproxy/{id} | 
[**api_v1_indexerproxy_post**](IndexerProxyApi.md#api_v1_indexerproxy_post) | **POST** /api/v1/indexerproxy | 
[**api_v1_indexerproxy_schema_get**](IndexerProxyApi.md#api_v1_indexerproxy_schema_get) | **GET** /api/v1/indexerproxy/schema | 
[**api_v1_indexerproxy_test_post**](IndexerProxyApi.md#api_v1_indexerproxy_test_post) | **POST** /api/v1/indexerproxy/test | 
[**api_v1_indexerproxy_testall_post**](IndexerProxyApi.md#api_v1_indexerproxy_testall_post) | **POST** /api/v1/indexerproxy/testall | 



## api_v1_indexerproxy_action_name_post

> api_v1_indexerproxy_action_name_post(name, indexer_proxy_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**indexer_proxy_resource** | Option<[**IndexerProxyResource**](IndexerProxyResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_indexerproxy_get

> Vec<models::IndexerProxyResource> api_v1_indexerproxy_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::IndexerProxyResource>**](IndexerProxyResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_indexerproxy_id_delete

> api_v1_indexerproxy_id_delete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_indexerproxy_id_get

> models::IndexerProxyResource api_v1_indexerproxy_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::IndexerProxyResource**](IndexerProxyResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_indexerproxy_id_put

> models::IndexerProxyResource api_v1_indexerproxy_id_put(id, force_save, indexer_proxy_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**force_save** | Option<**bool**> |  |  |[default to false]
**indexer_proxy_resource** | Option<[**IndexerProxyResource**](IndexerProxyResource.md)> |  |  |

### Return type

[**models::IndexerProxyResource**](IndexerProxyResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_indexerproxy_post

> models::IndexerProxyResource api_v1_indexerproxy_post(force_save, indexer_proxy_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_save** | Option<**bool**> |  |  |[default to false]
**indexer_proxy_resource** | Option<[**IndexerProxyResource**](IndexerProxyResource.md)> |  |  |

### Return type

[**models::IndexerProxyResource**](IndexerProxyResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_indexerproxy_schema_get

> Vec<models::IndexerProxyResource> api_v1_indexerproxy_schema_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::IndexerProxyResource>**](IndexerProxyResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_indexerproxy_test_post

> api_v1_indexerproxy_test_post(force_test, indexer_proxy_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_test** | Option<**bool**> |  |  |[default to false]
**indexer_proxy_resource** | Option<[**IndexerProxyResource**](IndexerProxyResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_indexerproxy_testall_post

> api_v1_indexerproxy_testall_post()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

