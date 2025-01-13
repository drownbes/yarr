# \LogFileApi

All URIs are relative to *http://localhost:9696*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v1_log_file_filename_get**](LogFileApi.md#api_v1_log_file_filename_get) | **GET** /api/v1/log/file/{filename} | 
[**api_v1_log_file_get**](LogFileApi.md#api_v1_log_file_get) | **GET** /api/v1/log/file | 



## api_v1_log_file_filename_get

> serde_json::Value api_v1_log_file_filename_get(filename)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filename** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_log_file_get

> Vec<models::LogFileResource> api_v1_log_file_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::LogFileResource>**](LogFileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

