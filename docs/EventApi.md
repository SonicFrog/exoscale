# \EventApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_events**](EventApi.md#list_events) | **GET** /event | List Events



## list_events

> Vec<crate::models::Event> list_events(from, to)
List Events

Retrieve Events for a given date range. Defaults to retrieving Events           for the current and previous day. Both a `from` and `to` arguments can           be passed specifying dates to start from and to

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | Option<**String**> |  |  |
**to** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::Event>**](event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

