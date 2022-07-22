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
pub struct DbaasServiceMysqlUsersInner {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "authentication", skip_serializing_if = "Option::is_none")]
    pub authentication: Option<String>,
}

impl DbaasServiceMysqlUsersInner {
    pub fn new() -> DbaasServiceMysqlUsersInner {
        DbaasServiceMysqlUsersInner {
            _type: None,
            username: None,
            password: None,
            authentication: None,
        }
    }
}
