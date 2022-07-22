/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://bump.sh/doc/exoscale-api.json) * [YAML format](https://bump.sh/doc/exoscale-api.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */

/// Quota : Organization Quota

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Quota {
    /// Resource Name
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// Resource Usage
    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<i64>,
    /// Resource Limit. -1 for Unlimited
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl Quota {
    /// Organization Quota
    pub fn new() -> Quota {
        Quota {
            resource: None,
            usage: None,
            limit: None,
        }
    }
}
