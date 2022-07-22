/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://bump.sh/doc/exoscale-api.json) * [YAML format](https://bump.sh/doc/exoscale-api.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */

/// LoadBalancerService : Load Balancer Service

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoadBalancerService {
    /// Load Balancer Service description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Network traffic protocol
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Protocol>,
    /// Load Balancer Service name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Load Balancer Service state
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// Port on which the network traffic will be forwarded to on the receiving instance
    #[serde(rename = "target-port", skip_serializing_if = "Option::is_none")]
    pub target_port: Option<i64>,
    /// Port exposed on the Load Balancer's public IP
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(rename = "instance-pool", skip_serializing_if = "Option::is_none")]
    pub instance_pool: Option<Box<crate::models::InstancePool>>,
    /// Load balancing strategy
    #[serde(rename = "strategy", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<Strategy>,
    #[serde(rename = "healthcheck", skip_serializing_if = "Option::is_none")]
    pub healthcheck: Option<Box<crate::models::LoadBalancerServiceHealthcheck>>,
    /// Load Balancer Service ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Healthcheck status per backend server
    #[serde(rename = "healthcheck-status", skip_serializing_if = "Option::is_none")]
    pub healthcheck_status: Option<Vec<crate::models::LoadBalancerServerStatus>>,
}

impl LoadBalancerService {
    /// Load Balancer Service
    pub fn new() -> LoadBalancerService {
        LoadBalancerService {
            description: None,
            protocol: None,
            name: None,
            state: None,
            target_port: None,
            port: None,
            instance_pool: None,
            strategy: None,
            healthcheck: None,
            id: None,
            healthcheck_status: None,
        }
    }
}

/// Network traffic protocol
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Protocol {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
}

impl Default for Protocol {
    fn default() -> Protocol {
        Self::Tcp
    }
}
/// Load Balancer Service state
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "creating")]
    Creating,
    #[serde(rename = "deleting")]
    Deleting,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "updating")]
    Updating,
    #[serde(rename = "error")]
    Error,
}

impl Default for State {
    fn default() -> State {
        Self::Creating
    }
}
/// Load balancing strategy
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Strategy {
    #[serde(rename = "round-robin")]
    RoundRobin,
    #[serde(rename = "source-hash")]
    SourceHash,
}

impl Default for Strategy {
    fn default() -> Strategy {
        Self::RoundRobin
    }
}
