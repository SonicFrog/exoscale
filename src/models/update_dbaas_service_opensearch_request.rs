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
pub struct UpdateDbaasServiceOpensearchRequest {
    /// Maximum number of indexes to keep before deleting the oldest one
    #[serde(rename = "max-index-count", skip_serializing_if = "Option::is_none")]
    pub max_index_count: Option<i64>,
    /// Aiven automation resets index.refresh_interval to default value for every index to be sure that indices are always visible to search. If it doesn't fit your case, you can disable this by setting up this flag to true.
    #[serde(
        rename = "keep-index-refresh-interval",
        skip_serializing_if = "Option::is_none"
    )]
    pub keep_index_refresh_interval: Option<bool>,
    /// Allow incoming connections from CIDR address block, e.g. '10.20.0.0/16'
    #[serde(rename = "ip-filter", skip_serializing_if = "Option::is_none")]
    pub ip_filter: Option<Vec<String>>,
    /// Service is protected against termination and powering off
    #[serde(
        rename = "termination-protection",
        skip_serializing_if = "Option::is_none"
    )]
    pub termination_protection: Option<bool>,
    /// Allows you to create glob style patterns and set a max number of indexes matching this pattern you want to keep. Creating indexes exceeding this value will cause the oldest one to get deleted. You could for example create a pattern looking like 'logs.?' and then create index logs.1, logs.2 etc, it will delete logs.1 once you create logs.6. Do note 'logs.?' does not apply to logs.10. Note: Setting max_index_count to 0 will do nothing and the pattern gets ignored.
    #[serde(rename = "index-patterns", skip_serializing_if = "Option::is_none")]
    pub index_patterns:
        Option<Vec<crate::models::UpdateDbaasServiceOpensearchRequestIndexPatternsInner>>,
    #[serde(rename = "maintenance", skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<Box<crate::models::UpdateDbaasServiceMysqlRequestMaintenance>>,
    #[serde(rename = "index-template", skip_serializing_if = "Option::is_none")]
    pub index_template:
        Option<Box<crate::models::UpdateDbaasServiceOpensearchRequestIndexTemplate>>,
    /// OpenSearch-specific settings
    #[serde(
        rename = "opensearch-settings",
        skip_serializing_if = "Option::is_none"
    )]
    pub opensearch_settings: Option<serde_json::Value>,
    /// Subscription plan
    #[serde(rename = "plan", skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(
        rename = "opensearch-dashboards",
        skip_serializing_if = "Option::is_none"
    )]
    pub opensearch_dashboards:
        Option<Box<crate::models::UpdateDbaasServiceOpensearchRequestOpensearchDashboards>>,
}

impl UpdateDbaasServiceOpensearchRequest {
    pub fn new() -> UpdateDbaasServiceOpensearchRequest {
        UpdateDbaasServiceOpensearchRequest {
            max_index_count: None,
            keep_index_refresh_interval: None,
            ip_filter: None,
            termination_protection: None,
            index_patterns: None,
            maintenance: None,
            index_template: None,
            opensearch_settings: None,
            plan: None,
            opensearch_dashboards: None,
        }
    }
}
