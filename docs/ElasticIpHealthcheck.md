# ElasticIpHealthcheck

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**strikes_ok** | Option<**i64**> | Number of attempts before considering the target healthy (default: 2) | [optional]
**tls_skip_verify** | Option<**bool**> | Skip TLS verification | [optional]
**tls_sni** | Option<**String**> | SNI domain for HTTPS healthchecks | [optional]
**strikes_fail** | Option<**i64**> | Number of attempts before considering the target unhealthy (default: 3) | [optional]
**mode** | **String** | Healthcheck mode | 
**port** | **i64** | Healthcheck port | 
**uri** | Option<**String**> | Healthcheck URI | [optional]
**interval** | Option<**i64**> | Interval between the checks (default: 10) | [optional]
**timeout** | Option<**i64**> | Healthcheck timeout value (default: 2) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


