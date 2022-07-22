# \IamApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_access_key**](IamApi.md#create_access_key) | **POST** /access-key | Create an IAM Access Key
[**get_access_key**](IamApi.md#get_access_key) | **GET** /access-key/{key} | Retrieve IAM Access Key details
[**list_access_key_known_operations**](IamApi.md#list_access_key_known_operations) | **GET** /access-key-known-operations | Retrieve all known available IAM Access Key operations and associated tags
[**list_access_key_operations**](IamApi.md#list_access_key_operations) | **GET** /access-key-operations | Retrieve IAM Access Key operations and associated tags for the signing key
[**list_access_keys**](IamApi.md#list_access_keys) | **GET** /access-key | List IAM Access Keys
[**revoke_access_key**](IamApi.md#revoke_access_key) | **DELETE** /access-key/{key} | Revoke an IAM Access Key



## create_access_key

> crate::models::AccessKey create_access_key(create_access_key_request)
Create an IAM Access Key

This operation creates a new IAM Access Key, which can either be restricted to specific API operations or unrestricted. The corresponding secret is only available in the response returned by this operation, the caller must take care of storing it safely as there is no other way to retrieve it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_access_key_request** | [**CreateAccessKeyRequest**](CreateAccessKeyRequest.md) |  | [required] |

### Return type

[**crate::models::AccessKey**](access-key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_access_key

> crate::models::AccessKey get_access_key(key)
Retrieve IAM Access Key details



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |

### Return type

[**crate::models::AccessKey**](access-key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_access_key_known_operations

> crate::models::ListAccessKeyKnownOperations200Response list_access_key_known_operations()
Retrieve all known available IAM Access Key operations and associated tags



### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListAccessKeyKnownOperations200Response**](list_access_key_known_operations_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_access_key_operations

> crate::models::ListAccessKeyKnownOperations200Response list_access_key_operations()
Retrieve IAM Access Key operations and associated tags for the signing key



### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListAccessKeyKnownOperations200Response**](list_access_key_known_operations_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_access_keys

> crate::models::ListAccessKeys200Response list_access_keys()
List IAM Access Keys



### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListAccessKeys200Response**](list_access_keys_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_access_key

> crate::models::Operation revoke_access_key(key)
Revoke an IAM Access Key

This operation revokes the specified IAM Access Key. Access Keys created by the revoked Access Key will not be revoked.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

