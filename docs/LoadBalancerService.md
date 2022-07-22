# LoadBalancerService

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Load Balancer Service description | [optional]
**protocol** | Option<**String**> | Network traffic protocol | [optional]
**name** | Option<**String**> | Load Balancer Service name | [optional]
**state** | Option<**String**> | Load Balancer Service state | [optional][readonly]
**target_port** | Option<**i64**> | Port on which the network traffic will be forwarded to on the receiving instance | [optional]
**port** | Option<**i64**> | Port exposed on the Load Balancer's public IP | [optional]
**instance_pool** | Option<[**crate::models::InstancePool**](instance-pool.md)> |  | [optional]
**strategy** | Option<**String**> | Load balancing strategy | [optional]
**healthcheck** | Option<[**crate::models::LoadBalancerServiceHealthcheck**](load-balancer-service-healthcheck.md)> |  | [optional]
**id** | Option<**String**> | Load Balancer Service ID | [optional][readonly]
**healthcheck_status** | Option<[**Vec<crate::models::LoadBalancerServerStatus>**](load-balancer-server-status.md)> | Healthcheck status per backend server | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


