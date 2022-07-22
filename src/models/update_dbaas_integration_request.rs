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
pub struct UpdateDbaasIntegrationRequest {
    /// Integration settings
    #[serde(rename = "settings")]
    pub settings: serde_json::Value,
}

impl UpdateDbaasIntegrationRequest {
    pub fn new(settings: serde_json::Value) -> UpdateDbaasIntegrationRequest {
        UpdateDbaasIntegrationRequest { settings }
    }
}
