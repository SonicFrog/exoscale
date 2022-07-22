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
pub struct UpdateDbaasServiceMysqlRequestBackupSchedule {
    /// The hour of day (in UTC) when backup for the service is started. New backup is only started if previous backup has already completed.
    #[serde(rename = "backup-hour", skip_serializing_if = "Option::is_none")]
    pub backup_hour: Option<i64>,
    /// The minute of an hour when backup for the service is started. New backup is only started if previous backup has already completed.
    #[serde(rename = "backup-minute", skip_serializing_if = "Option::is_none")]
    pub backup_minute: Option<i64>,
}

impl UpdateDbaasServiceMysqlRequestBackupSchedule {
    pub fn new() -> UpdateDbaasServiceMysqlRequestBackupSchedule {
        UpdateDbaasServiceMysqlRequestBackupSchedule {
            backup_hour: None,
            backup_minute: None,
        }
    }
}
