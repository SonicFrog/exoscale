# CreateDbaasServiceRedisRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**integrations** | Option<[**Vec<crate::models::CreateDbaasServiceMysqlRequestIntegrationsInner>**](create_dbaas_service_mysql_request_integrations_inner.md)> | Service integrations to enable for the service. Some integration types affect how a service is created and they must be provided as part of the creation call instead of being defined later. | [optional]
**redis_settings** | Option<[**serde_json::Value**](.md)> | Redis.conf settings | [optional]
**ip_filter** | Option<**Vec<String>**> | Allow incoming connections from CIDR address block, e.g. '10.20.0.0/16' | [optional]
**termination_protection** | Option<**bool**> | Service is protected against termination and powering off | [optional]
**fork_from_service** | Option<**String**> |  | [optional]
**maintenance** | Option<[**crate::models::UpdateDbaasServiceMysqlRequestMaintenance**](update_dbaas_service_mysql_request_maintenance.md)> |  | [optional]
**recovery_backup_name** | Option<**String**> | Name of a backup to recover from for services that support backup names | [optional]
**plan** | **String** | Subscription plan | 
**migration** | Option<[**crate::models::UpdateDbaasServiceMysqlRequestMigration**](update_dbaas_service_mysql_request_migration.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


