/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://bump.sh/doc/exoscale-api.json) * [YAML format](https://bump.sh/doc/exoscale-api.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */

/// ElasticIpHealthcheck : Elastic IP address healthcheck

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ElasticIpHealthcheck {
    /// Number of attempts before considering the target healthy (default: 2)
    #[serde(rename = "strikes-ok", skip_serializing_if = "Option::is_none")]
    pub strikes_ok: Option<i64>,
    /// Skip TLS verification
    #[serde(rename = "tls-skip-verify", skip_serializing_if = "Option::is_none")]
    pub tls_skip_verify: Option<bool>,
    /// SNI domain for HTTPS healthchecks
    #[serde(rename = "tls-sni", skip_serializing_if = "Option::is_none")]
    pub tls_sni: Option<String>,
    /// Number of attempts before considering the target unhealthy (default: 3)
    #[serde(rename = "strikes-fail", skip_serializing_if = "Option::is_none")]
    pub strikes_fail: Option<i64>,
    /// Healthcheck mode
    #[serde(rename = "mode")]
    pub mode: Mode,
    /// Healthcheck port
    #[serde(rename = "port")]
    pub port: i64,
    /// Healthcheck URI
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// Interval between the checks (default: 10)
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    /// Healthcheck timeout value (default: 2)
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}

impl ElasticIpHealthcheck {
    /// Elastic IP address healthcheck
    pub fn new(mode: Mode, port: i64) -> ElasticIpHealthcheck {
        ElasticIpHealthcheck {
            strikes_ok: None,
            tls_skip_verify: None,
            tls_sni: None,
            strikes_fail: None,
            mode,
            port,
            uri: None,
            interval: None,
            timeout: None,
        }
    }
}

/// Healthcheck mode
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "https")]
    Https,
}

impl Default for Mode {
    fn default() -> Mode {
        Self::Tcp
    }
}
