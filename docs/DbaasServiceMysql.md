# DbaasServiceMysql

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**updated_at** | Option<**String**> | Service last update timestamp (ISO 8601) | [optional]
**node_count** | Option<**i64**> | Number of service nodes in the active plan | [optional]
**connection_info** | Option<[**crate::models::DbaasServiceMysqlConnectionInfo**](dbaas_service_mysql_connection_info.md)> |  | [optional]
**backup_schedule** | Option<[**crate::models::DbaasServiceMysqlBackupSchedule**](dbaas_service_mysql_backup_schedule.md)> |  | [optional]
**node_cpu_count** | Option<**i64**> | Number of CPUs for each node | [optional]
**integrations** | Option<[**Vec<crate::models::DbaasIntegration>**](dbaas-integration.md)> | Service integrations | [optional]
**node_states** | Option<[**Vec<crate::models::DbaasNodeState>**](dbaas-node-state.md)> | State of individual service nodes | [optional]
**name** | **String** |  | 
**_type** | **String** |  | 
**state** | Option<[**crate::models::EnumServiceState**](enum-service-state.md)> |  | [optional]
**ip_filter** | Option<**Vec<String>**> | Allowed CIDR address blocks for incoming connections | [optional]
**backups** | Option<[**Vec<crate::models::DbaasServiceBackup>**](dbaas-service-backup.md)> | List of backups for the service | [optional]
**termination_protection** | Option<**bool**> | Service is protected against termination and powering off | [optional]
**notifications** | Option<[**Vec<crate::models::DbaasServiceNotification>**](dbaas-service-notification.md)> | Service notifications | [optional]
**components** | Option<[**Vec<crate::models::DbaasServiceMysqlComponentsInner>**](dbaas_service_mysql_components_inner.md)> | Service component information objects | [optional]
**mysql_settings** | Option<[**serde_json::Value**](.md)> | MySQL-specific settings | [optional]
**maintenance** | Option<[**crate::models::DbaasServiceMaintenance**](dbaas-service-maintenance.md)> |  | [optional]
**disk_size** | Option<**i64**> | TODO UNIT disk space for data storage | [optional]
**node_memory** | Option<**i64**> | TODO UNIT of memory for each node | [optional]
**uri** | Option<**String**> | URI for connecting to the service (may be absent) | [optional]
**uri_params** | Option<[**serde_json::Value**](.md)> | service_uri parameterized into key-value pairs | [optional]
**version** | Option<**String**> | MySQL version | [optional]
**created_at** | Option<**String**> | Service creation timestamp (ISO 8601) | [optional]
**plan** | **String** | Subscription plan | 
**users** | Option<[**Vec<crate::models::DbaasServiceMysqlUsersInner>**](dbaas_service_mysql_users_inner.md)> | List of service users | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


