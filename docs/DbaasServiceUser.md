# DbaasServiceUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_cert** | Option<**String**> | Access certificate for TLS client authentication | [optional]
**access_cert_not_valid_after_time** | Option<**String**> | Access certificate validity end time (ISO8601) | [optional]
**access_control** | Option<[**crate::models::DbaasServiceUserAccessControl**](dbaas-service-user-access-control.md)> |  | [optional]
**access_key** | Option<**String**> | Service specific access controls for user | [optional]
**authentication** | Option<**String**> | Access key for TLS client authentication | [optional]
**password** | Option<**String**> | Account password. A missing field indicates a user overridden password. | [optional]
**_type** | **String** | Account type | 
**username** | **String** | Account username | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


