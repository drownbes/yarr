# \AppProfileApi

All URIs are relative to *http://localhost:9696*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v1_appprofile_get**](AppProfileApi.md#api_v1_appprofile_get) | **GET** /api/v1/appprofile | 
[**api_v1_appprofile_id_delete**](AppProfileApi.md#api_v1_appprofile_id_delete) | **DELETE** /api/v1/appprofile/{id} | 
[**api_v1_appprofile_id_get**](AppProfileApi.md#api_v1_appprofile_id_get) | **GET** /api/v1/appprofile/{id} | 
[**api_v1_appprofile_id_put**](AppProfileApi.md#api_v1_appprofile_id_put) | **PUT** /api/v1/appprofile/{id} | 
[**api_v1_appprofile_post**](AppProfileApi.md#api_v1_appprofile_post) | **POST** /api/v1/appprofile | 
[**api_v1_appprofile_schema_get**](AppProfileApi.md#api_v1_appprofile_schema_get) | **GET** /api/v1/appprofile/schema | 



## api_v1_appprofile_get

> Vec<models::AppProfileResource> api_v1_appprofile_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::AppProfileResource>**](AppProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_appprofile_id_delete

> api_v1_appprofile_id_delete(id)


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


## api_v1_appprofile_id_get

> models::AppProfileResource api_v1_appprofile_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::AppProfileResource**](AppProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_appprofile_id_put

> models::AppProfileResource api_v1_appprofile_id_put(id, app_profile_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**app_profile_resource** | Option<[**AppProfileResource**](AppProfileResource.md)> |  |  |

### Return type

[**models::AppProfileResource**](AppProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_appprofile_post

> models::AppProfileResource api_v1_appprofile_post(app_profile_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_profile_resource** | Option<[**AppProfileResource**](AppProfileResource.md)> |  |  |

### Return type

[**models::AppProfileResource**](AppProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_appprofile_schema_get

> models::AppProfileResource api_v1_appprofile_schema_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AppProfileResource**](AppProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

