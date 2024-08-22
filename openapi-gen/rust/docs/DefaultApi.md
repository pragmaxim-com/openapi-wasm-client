# \DefaultApi

All URIs are relative to *http://localhost:8082*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_addresses**](DefaultApi.md#get_addresses) | **GET** /addresses | Get addresses
[**get_blocks**](DefaultApi.md#get_blocks) | **GET** /blocks | Get blocks
[**store_address**](DefaultApi.md#store_address) | **POST** /addresses | Store an address
[**store_block**](DefaultApi.md#store_block) | **POST** /blocks | Store a block



## get_addresses

> Vec<models::Address> get_addresses()
Get addresses

Retrieve a list of addresses

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Address>**](Address.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blocks

> Vec<models::Block> get_blocks()
Get blocks

Retrieve a list of blocks

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Block>**](Block.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_address

> store_address(address1)
Store an address

Store a new address in the system

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address1** | [**Address1**](Address1.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_block

> store_block(block1)
Store a block

Store a new block in the system

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block1** | [**Block1**](Block1.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

