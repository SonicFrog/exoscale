# DbaasServiceKafka

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**updated_at** | Option<**String**> | Service last update timestamp (ISO 8601) | [optional]
**authentication_methods** | Option<[**crate::models::DbaasServiceKafkaAuthenticationMethods**](dbaas_service_kafka_authentication_methods.md)> |  | [optional]
**node_count** | Option<**i64**> | Number of service nodes in the active plan | [optional]
**connection_info** | Option<[**crate::models::DbaasServiceKafkaConnectionInfo**](dbaas_service_kafka_connection_info.md)> |  | [optional]
**node_cpu_count** | Option<**i64**> | Number of CPUs for each node | [optional]
**kafka_rest_enabled** | Option<**bool**> | Whether Kafka REST is enabled | [optional]
**integrations** | Option<[**Vec<crate::models::DbaasIntegration>**](dbaas-integration.md)> | Service integrations | [optional]
**node_states** | Option<[**Vec<crate::models::DbaasNodeState>**](dbaas-node-state.md)> | State of individual service nodes | [optional]
**name** | **String** |  | 
**kafka_connect_enabled** | Option<**bool**> | Whether Kafka Connect is enabled | [optional]
**_type** | **String** |  | 
**state** | Option<[**crate::models::EnumServiceState**](enum-service-state.md)> |  | [optional]
**ip_filter** | Option<**Vec<String>**> | Allow incoming connections from CIDR address block, e.g. '10.20.0.0/16' | [optional]
**schema_registry_settings** | Option<[**serde_json::Value**](.md)> | Schema Registry configuration | [optional]
**backups** | Option<[**Vec<crate::models::DbaasServiceBackup>**](dbaas-service-backup.md)> | List of backups for the service | [optional]
**kafka_rest_settings** | Option<[**serde_json::Value**](.md)> | Kafka REST configuration | [optional]
**termination_protection** | Option<**bool**> | Service is protected against termination and powering off | [optional]
**notifications** | Option<[**Vec<crate::models::DbaasServiceNotification>**](dbaas-service-notification.md)> | Service notifications | [optional]
**kafka_connect_settings** | Option<[**serde_json::Value**](.md)> | Kafka Connect configuration values | [optional]
**components** | Option<[**Vec<crate::models::DbaasServiceKafkaComponentsInner>**](dbaas_service_kafka_components_inner.md)> | Service component information objects | [optional]
**maintenance** | Option<[**crate::models::DbaasServiceMaintenance**](dbaas-service-maintenance.md)> |  | [optional]
**kafka_settings** | Option<[**serde_json::Value**](.md)> | Kafka-specific settings | [optional]
**acl** | Option<[**Vec<crate::models::DbaasServiceKafkaAclInner>**](dbaas_service_kafka_acl_inner.md)> | List of Kafka ACL entries | [optional]
**disk_size** | Option<**i64**> | TODO UNIT disk space for data storage | [optional]
**node_memory** | Option<**i64**> | TODO UNIT of memory for each node | [optional]
**uri** | Option<**String**> | URI for connecting to the service (may be absent) | [optional]
**uri_params** | Option<[**serde_json::Value**](.md)> | service_uri parameterized into key-value pairs | [optional]
**schema_registry_enabled** | Option<**bool**> | Whether Schema-Registry is enabled | [optional]
**version** | Option<**String**> | Kafka version | [optional]
**created_at** | Option<**String**> | Service creation timestamp (ISO 8601) | [optional]
**plan** | **String** | Subscription plan | 
**users** | Option<[**Vec<crate::models::DbaasServiceKafkaUsersInner>**](dbaas_service_kafka_users_inner.md)> | List of service users | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


