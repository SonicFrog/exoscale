/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://bump.sh/doc/exoscale-api.json) * [YAML format](https://bump.sh/doc/exoscale-api.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */

/// DbaasServiceIntegration : Integrations with other services

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DbaasServiceIntegration {
    /// Integration status
    #[serde(rename = "integration-status", skip_serializing_if = "Option::is_none")]
    pub integration_status: Option<serde_json::Value>,
    /// Description of the integration
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "source-service-type")]
    pub source_service_type: String,
    /// Source endpoint name
    #[serde(rename = "source-endpoint", skip_serializing_if = "Option::is_none")]
    pub source_endpoint: Option<String>,
    #[serde(rename = "dest-service-type")]
    pub dest_service_type: String,
    /// Type of the integration
    #[serde(rename = "integration-type")]
    pub integration_type: String,
    /// Destination endpoint name
    #[serde(rename = "dest-endpoint", skip_serializing_if = "Option::is_none")]
    pub dest_endpoint: Option<String>,
    /// Service integration settings
    #[serde(rename = "user-config", skip_serializing_if = "Option::is_none")]
    pub user_config: Option<serde_json::Value>,
    /// Destination endpoint id
    #[serde(rename = "dest-endpoint-id", skip_serializing_if = "Option::is_none")]
    pub dest_endpoint_id: Option<String>,
    /// Integration ID
    #[serde(rename = "service-integration-id")]
    pub service_integration_id: String,
    /// Destination service name
    #[serde(rename = "dest-service")]
    pub dest_service: String,
    /// True when integration is active
    #[serde(rename = "active")]
    pub active: bool,
    /// Source endpoint ID
    #[serde(rename = "source-endpoint-id", skip_serializing_if = "Option::is_none")]
    pub source_endpoint_id: Option<String>,
    /// Source service name
    #[serde(rename = "source-service")]
    pub source_service: String,
    /// True when integration is enabled
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

impl DbaasServiceIntegration {
    /// Integrations with other services
    pub fn new(
        description: String,
        source_service_type: String,
        dest_service_type: String,
        integration_type: String,
        service_integration_id: String,
        dest_service: String,
        active: bool,
        source_service: String,
        enabled: bool,
    ) -> DbaasServiceIntegration {
        DbaasServiceIntegration {
            integration_status: None,
            description,
            source_service_type,
            source_endpoint: None,
            dest_service_type,
            integration_type,
            dest_endpoint: None,
            user_config: None,
            dest_endpoint_id: None,
            service_integration_id,
            dest_service,
            active,
            source_endpoint_id: None,
            source_service,
            enabled,
        }
    }
}
