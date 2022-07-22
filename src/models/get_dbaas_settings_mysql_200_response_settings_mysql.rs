/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://bump.sh/doc/exoscale-api.json) * [YAML format](https://bump.sh/doc/exoscale-api.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */

/// GetDbaasSettingsMysql200ResponseSettingsMysql : mysql.conf configuration values

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetDbaasSettingsMysql200ResponseSettingsMysql {
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(
        rename = "additionalProperties",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_properties: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl GetDbaasSettingsMysql200ResponseSettingsMysql {
    /// mysql.conf configuration values
    pub fn new() -> GetDbaasSettingsMysql200ResponseSettingsMysql {
        GetDbaasSettingsMysql200ResponseSettingsMysql {
            properties: None,
            additional_properties: None,
            _type: None,
            title: None,
        }
    }
}
