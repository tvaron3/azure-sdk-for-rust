#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsList {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartDetectionMigrationRequest {
    pub scope: Vec<String>,
    #[serde(rename = "actionGroupCreationPolicy", default, skip_serializing_if = "Option::is_none")]
    pub action_group_creation_policy: Option<smart_detection_migration_request::ActionGroupCreationPolicy>,
    #[serde(rename = "customActionGroupName", default, skip_serializing_if = "Option::is_none")]
    pub custom_action_group_name: Option<String>,
}
pub mod smart_detection_migration_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ActionGroupCreationPolicy {
        Custom,
        Auto,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrationStatusResponse {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MigrationStatusResponseProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrationStatusResponseProperties {
    #[serde(rename = "migrationId")]
    pub migration_id: String,
    pub status: migration_status_response_properties::Status,
    pub scope: Vec<String>,
    #[serde(rename = "armDeploymentName", default, skip_serializing_if = "Option::is_none")]
    pub arm_deployment_name: Option<String>,
}
pub mod migration_status_response_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Starting,
        InProcess,
        Deploying,
        Completed,
        Failed,
        Canceled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrationErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<MigrationErrorResponseBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrationErrorResponseBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}