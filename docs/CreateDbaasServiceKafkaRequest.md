# CreateDbaasServiceKafkaRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authentication_methods** | Option<[**crate::models::UpdateDbaasServiceKafkaRequestAuthenticationMethods**](update_dbaas_service_kafka_request_authentication_methods.md)> |  | [optional]
**kafka_rest_enabled** | Option<**bool**> | Enable Kafka-REST service | [optional]
**integrations** | Option<[**Vec<crate::models::CreateDbaasServiceMysqlRequestIntegrationsInner>**](create_dbaas_service_mysql_request_integrations_inner.md)> | Service integrations to enable for the service. Some integration types affect how a service is created and they must be provided as part of the creation call instead of being defined later. | [optional]
**kafka_connect_enabled** | Option<**bool**> | Allow clients to connect to kafka_connect from the public internet for service nodes that are in a project VPC or another type of private network | [optional]
**ip_filter** | Option<**Vec<String>**> | Allow incoming connections from CIDR address block, e.g. '10.20.0.0/16' | [optional]
**schema_registry_settings** | Option<[**serde_json::Value**](.md)> | Schema Registry configuration | [optional]
**kafka_rest_settings** | Option<[**serde_json::Value**](.md)> | Kafka REST configuration | [optional]
**termination_protection** | Option<**bool**> | Service is protected against termination and powering off | [optional]
**kafka_connect_settings** | Option<[**serde_json::Value**](.md)> | Kafka Connect configuration values | [optional]
**maintenance** | Option<[**crate::models::UpdateDbaasServiceMysqlRequestMaintenance**](update_dbaas_service_mysql_request_maintenance.md)> |  | [optional]
**kafka_settings** | Option<[**serde_json::Value**](.md)> | Kafka-specific settings | [optional]
**schema_registry_enabled** | Option<**bool**> | Enable Schema-Registry service | [optional]
**version** | Option<**String**> | Kafka major version | [optional]
**plan** | **String** | Subscription plan | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


