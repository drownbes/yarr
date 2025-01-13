# \ApplicationApi

All URIs are relative to *http://localhost:9696*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v1_applications_action_name_post**](ApplicationApi.md#api_v1_applications_action_name_post) | **POST** /api/v1/applications/action/{name} | 
[**api_v1_applications_bulk_delete**](ApplicationApi.md#api_v1_applications_bulk_delete) | **DELETE** /api/v1/applications/bulk | 
[**api_v1_applications_bulk_put**](ApplicationApi.md#api_v1_applications_bulk_put) | **PUT** /api/v1/applications/bulk | 
[**api_v1_applications_get**](ApplicationApi.md#api_v1_applications_get) | **GET** /api/v1/applications | 
[**api_v1_applications_id_delete**](ApplicationApi.md#api_v1_applications_id_delete) | **DELETE** /api/v1/applications/{id} | 
[**api_v1_applications_id_get**](ApplicationApi.md#api_v1_applications_id_get) | **GET** /api/v1/applications/{id} | 
[**api_v1_applications_id_put**](ApplicationApi.md#api_v1_applications_id_put) | **PUT** /api/v1/applications/{id} | 
[**api_v1_applications_post**](ApplicationApi.md#api_v1_applications_post) | **POST** /api/v1/applications | 
[**api_v1_applications_schema_get**](ApplicationApi.md#api_v1_applications_schema_get) | **GET** /api/v1/applications/schema | 
[**api_v1_applications_test_post**](ApplicationApi.md#api_v1_applications_test_post) | **POST** /api/v1/applications/test | 
[**api_v1_applications_testall_post**](ApplicationApi.md#api_v1_applications_testall_post) | **POST** /api/v1/applications/testall | 



## api_v1_applications_action_name_post

> api_v1_applications_action_name_post(name, application_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**application_resource** | Option<[**ApplicationResource**](ApplicationResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_applications_bulk_delete

> api_v1_applications_bulk_delete(application_bulk_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_bulk_resource** | Option<[**ApplicationBulkResource**](ApplicationBulkResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_applications_bulk_put

> models::ApplicationResource api_v1_applications_bulk_put(application_bulk_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_bulk_resource** | Option<[**ApplicationBulkResource**](ApplicationBulkResource.md)> |  |  |

### Return type

[**models::ApplicationResource**](ApplicationResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_applications_get

> Vec<models::ApplicationResource> api_v1_applications_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ApplicationResource>**](ApplicationResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_applications_id_delete

> api_v1_applications_id_delete(id)


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


## api_v1_applications_id_get

> models::ApplicationResource api_v1_applications_id_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::ApplicationResource**](ApplicationResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_applications_id_put

> models::ApplicationResource api_v1_applications_id_put(id, force_save, application_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**force_save** | Option<**bool**> |  |  |[default to false]
**application_resource** | Option<[**ApplicationResource**](ApplicationResource.md)> |  |  |

### Return type

[**models::ApplicationResource**](ApplicationResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_applications_post

> models::ApplicationResource api_v1_applications_post(force_save, application_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_save** | Option<**bool**> |  |  |[default to false]
**application_resource** | Option<[**ApplicationResource**](ApplicationResource.md)> |  |  |

### Return type

[**models::ApplicationResource**](ApplicationResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_applications_schema_get

> Vec<models::ApplicationResource> api_v1_applications_schema_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ApplicationResource>**](ApplicationResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_applications_test_post

> api_v1_applications_test_post(force_test, application_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_test** | Option<**bool**> |  |  |[default to false]
**application_resource** | Option<[**ApplicationResource**](ApplicationResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_applications_testall_post

> api_v1_applications_testall_post()


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

