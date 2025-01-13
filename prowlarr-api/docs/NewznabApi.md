# \NewznabApi

All URIs are relative to *http://localhost:9696*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_v1_indexer_id_download_get**](NewznabApi.md#api_v1_indexer_id_download_get) | **GET** /api/v1/indexer/{id}/download | 
[**api_v1_indexer_id_newznab_get**](NewznabApi.md#api_v1_indexer_id_newznab_get) | **GET** /api/v1/indexer/{id}/newznab | 
[**id_api_get**](NewznabApi.md#id_api_get) | **GET** /{id}/api | 
[**id_download_get**](NewznabApi.md#id_download_get) | **GET** /{id}/download | 



## api_v1_indexer_id_download_get

> api_v1_indexer_id_download_get(id, link, file)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**link** | Option<**String**> |  |  |
**file** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_indexer_id_newznab_get

> api_v1_indexer_id_newznab_get(id, t, q, cat, imdbid, tmdbid, extended, limit, offset, minage, maxage, minsize, maxsize, rid, tvmazeid, traktid, tvdbid, doubanid, season, ep, album, artist, label, track, year, genre, author, title, publisher, configured, source, host, server)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**t** | Option<**String**> |  |  |
**q** | Option<**String**> |  |  |
**cat** | Option<**String**> |  |  |
**imdbid** | Option<**String**> |  |  |
**tmdbid** | Option<**i32**> |  |  |
**extended** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |
**minage** | Option<**i32**> |  |  |
**maxage** | Option<**i32**> |  |  |
**minsize** | Option<**i64**> |  |  |
**maxsize** | Option<**i64**> |  |  |
**rid** | Option<**i32**> |  |  |
**tvmazeid** | Option<**i32**> |  |  |
**traktid** | Option<**i32**> |  |  |
**tvdbid** | Option<**i32**> |  |  |
**doubanid** | Option<**i32**> |  |  |
**season** | Option<**i32**> |  |  |
**ep** | Option<**String**> |  |  |
**album** | Option<**String**> |  |  |
**artist** | Option<**String**> |  |  |
**label** | Option<**String**> |  |  |
**track** | Option<**String**> |  |  |
**year** | Option<**i32**> |  |  |
**genre** | Option<**String**> |  |  |
**author** | Option<**String**> |  |  |
**title** | Option<**String**> |  |  |
**publisher** | Option<**String**> |  |  |
**configured** | Option<**String**> |  |  |
**source** | Option<**String**> |  |  |
**host** | Option<**String**> |  |  |
**server** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## id_api_get

> id_api_get(id, t, q, cat, imdbid, tmdbid, extended, limit, offset, minage, maxage, minsize, maxsize, rid, tvmazeid, traktid, tvdbid, doubanid, season, ep, album, artist, label, track, year, genre, author, title, publisher, configured, source, host, server)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**t** | Option<**String**> |  |  |
**q** | Option<**String**> |  |  |
**cat** | Option<**String**> |  |  |
**imdbid** | Option<**String**> |  |  |
**tmdbid** | Option<**i32**> |  |  |
**extended** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |
**minage** | Option<**i32**> |  |  |
**maxage** | Option<**i32**> |  |  |
**minsize** | Option<**i64**> |  |  |
**maxsize** | Option<**i64**> |  |  |
**rid** | Option<**i32**> |  |  |
**tvmazeid** | Option<**i32**> |  |  |
**traktid** | Option<**i32**> |  |  |
**tvdbid** | Option<**i32**> |  |  |
**doubanid** | Option<**i32**> |  |  |
**season** | Option<**i32**> |  |  |
**ep** | Option<**String**> |  |  |
**album** | Option<**String**> |  |  |
**artist** | Option<**String**> |  |  |
**label** | Option<**String**> |  |  |
**track** | Option<**String**> |  |  |
**year** | Option<**i32**> |  |  |
**genre** | Option<**String**> |  |  |
**author** | Option<**String**> |  |  |
**title** | Option<**String**> |  |  |
**publisher** | Option<**String**> |  |  |
**configured** | Option<**String**> |  |  |
**source** | Option<**String**> |  |  |
**host** | Option<**String**> |  |  |
**server** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## id_download_get

> id_download_get(id, link, file)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**link** | Option<**String**> |  |  |
**file** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

