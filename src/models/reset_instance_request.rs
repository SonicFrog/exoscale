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
pub struct ResetInstanceRequest {
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<Box<crate::models::Template>>,
    /// Instance disk size in GB
    #[serde(rename = "disk-size", skip_serializing_if = "Option::is_none")]
    pub disk_size: Option<i64>,
}

impl ResetInstanceRequest {
    pub fn new() -> ResetInstanceRequest {
        ResetInstanceRequest {
            template: None,
            disk_size: None,
        }
    }
}
