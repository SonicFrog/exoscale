/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://bump.sh/doc/exoscale-api.json) * [YAML format](https://bump.sh/doc/exoscale-api.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DbaasIntegrationType {
    /// The type of the integration.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// The description of the source service types.
    #[serde(rename = "source-description", skip_serializing_if = "Option::is_none")]
    pub source_description: Option<String>,
    /// A list of the source service types the integration supports.
    #[serde(
        rename = "source-service-types",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_service_types: Option<Vec<String>>,
    /// The description of the destination service types.
    #[serde(rename = "dest-description", skip_serializing_if = "Option::is_none")]
    pub dest_description: Option<String>,
    /// A list of the destination service types the integration supports.
    #[serde(rename = "dest-service-types", skip_serializing_if = "Option::is_none")]
    pub dest_service_types: Option<Vec<String>>,
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<Box<crate::models::DbaasIntegrationTypeSettings>>,
}

impl DbaasIntegrationType {
    pub fn new() -> DbaasIntegrationType {
        DbaasIntegrationType {
            _type: None,
            source_description: None,
            source_service_types: None,
            dest_description: None,
            dest_service_types: None,
            settings: None,
        }
    }
}
