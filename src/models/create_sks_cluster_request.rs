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
pub struct CreateSksClusterRequest {
    /// Cluster description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// Cluster CNI
    #[serde(rename = "cni", skip_serializing_if = "Option::is_none")]
    pub cni: Option<Cni>,
    /// Enable auto upgrade of the control plane to the latest patch version available
    #[serde(rename = "auto-upgrade", skip_serializing_if = "Option::is_none")]
    pub auto_upgrade: Option<bool>,
    #[serde(rename = "oidc", skip_serializing_if = "Option::is_none")]
    pub oidc: Option<Box<crate::models::SksOidc>>,
    /// Cluster name
    #[serde(rename = "name")]
    pub name: String,
    /// Cluster service level
    #[serde(rename = "level")]
    pub level: Level,
    /// Cluster addons
    #[serde(rename = "addons", skip_serializing_if = "Option::is_none")]
    pub addons: Option<std::collections::HashSet<Addons>>,
    /// Control plane Kubernetes version
    #[serde(rename = "version")]
    pub version: String,
}

impl CreateSksClusterRequest {
    pub fn new(name: String, level: Level, version: String) -> CreateSksClusterRequest {
        CreateSksClusterRequest {
            description: None,
            labels: None,
            cni: None,
            auto_upgrade: None,
            oidc: None,
            name,
            level,
            addons: None,
            version,
        }
    }
}

/// Cluster CNI
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Cni {
    #[serde(rename = "calico")]
    Calico,
    #[serde(rename = "cilium")]
    Cilium,
}

impl Default for Cni {
    fn default() -> Cni {
        Self::Calico
    }
}
/// Cluster service level
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Level {
    #[serde(rename = "starter")]
    Starter,
    #[serde(rename = "pro")]
    Pro,
}

impl Default for Level {
    fn default() -> Level {
        Self::Starter
    }
}
/// Cluster addons
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Addons {
    #[serde(rename = "exoscale-cloud-controller")]
    ExoscaleCloudController,
    #[serde(rename = "metrics-server")]
    MetricsServer,
}

impl Default for Addons {
    fn default() -> Addons {
        Self::ExoscaleCloudController
    }
}
