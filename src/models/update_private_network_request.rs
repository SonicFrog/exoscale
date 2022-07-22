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
pub struct UpdatePrivateNetworkRequest {
    /// Private Network name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Private Network description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Private Network netmask
    #[serde(rename = "netmask", skip_serializing_if = "Option::is_none")]
    pub netmask: Option<String>,
    /// Private Network start IP address
    #[serde(rename = "start-ip", skip_serializing_if = "Option::is_none")]
    pub start_ip: Option<String>,
    /// Private Network end IP address
    #[serde(rename = "end-ip", skip_serializing_if = "Option::is_none")]
    pub end_ip: Option<String>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
}

impl UpdatePrivateNetworkRequest {
    pub fn new() -> UpdatePrivateNetworkRequest {
        UpdatePrivateNetworkRequest {
            name: None,
            description: None,
            netmask: None,
            start_ip: None,
            end_ip: None,
            labels: None,
        }
    }
}
