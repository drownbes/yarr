# \DevelopmentConfigApi

All URIs are relative to *http://localhost:9696*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v1_config_development_get**](DevelopmentConfigApi.md#api_v1_config_development_get) | **GET** /api/v1/config/development | 
[**api_v1_config_development_id_get**](DevelopmentConfigApi.md#api_v1_config_development_id_get) | **GET** /api/v1/config/development/{id} | 
[**api_v1_config_development_id_put**](DevelopmentConfigApi.md#api_v1_config_development_id_put) | **PUT** /api/v1/config/development/{id} | 



## api_v1_config_development_get

> models::DevelopmentConfigResource api_v1_config_development_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::DevelopmentConfigResource**](DevelopmentConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_config_development_id_get

> models::DevelopmentConfigResource api_v1_config_development_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::DevelopmentConfigResource**](DevelopmentConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_config_development_id_put

> models::DevelopmentConfigResource api_v1_config_development_id_put(id, development_config_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**development_config_resource** | Option<[**DevelopmentConfigResource**](DevelopmentConfigResource.md)> |  |  |

### Return type

[**models::DevelopmentConfigResource**](DevelopmentConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

