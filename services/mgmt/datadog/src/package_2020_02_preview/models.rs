#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatadogAgreementProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(rename = "licenseTextLink", skip_serializing_if = "Option::is_none")]
    pub license_text_link: Option<String>,
    #[serde(rename = "privacyPolicyLink", skip_serializing_if = "Option::is_none")]
    pub privacy_policy_link: Option<String>,
    #[serde(rename = "retrieveDatetime", skip_serializing_if = "Option::is_none")]
    pub retrieve_datetime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatadogAgreementResource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatadogAgreementProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatadogAgreementResourceListResponse {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DatadogAgreementResource>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatadogApiKey {
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatadogApiKeyListResponse {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DatadogApiKey>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponseBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorResponseBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceProviderDefaultErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponseBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatadogInstallMethod {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool: Option<String>,
    #[serde(rename = "toolVersion", skip_serializing_if = "Option::is_none")]
    pub tool_version: Option<String>,
    #[serde(rename = "installerVersion", skip_serializing_if = "Option::is_none")]
    pub installer_version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatadogLogsAgent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatadogHostMetadata {
    #[serde(rename = "agentVersion", skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[serde(rename = "installMethod", skip_serializing_if = "Option::is_none")]
    pub install_method: Option<DatadogInstallMethod>,
    #[serde(rename = "logsAgent", skip_serializing_if = "Option::is_none")]
    pub logs_agent: Option<DatadogLogsAgent>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatadogHost {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub apps: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<DatadogHostMetadata>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatadogHostListResponse {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DatadogHost>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedResourceListResponse {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LinkedResource>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitoredResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "sendingMetrics", skip_serializing_if = "Option::is_none")]
    pub sending_metrics: Option<bool>,
    #[serde(rename = "reasonForMetricsStatus", skip_serializing_if = "Option::is_none")]
    pub reason_for_metrics_status: Option<String>,
    #[serde(rename = "sendingLogs", skip_serializing_if = "Option::is_none")]
    pub sending_logs: Option<bool>,
    #[serde(rename = "reasonForLogsStatus", skip_serializing_if = "Option::is_none")]
    pub reason_for_logs_status: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitoredResourceListResponse {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MonitoredResource>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDisplay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[serde(rename = "isDataAction", skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationResult>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSku {
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProvisioningState {
    Accepted,
    Creating,
    Updating,
    Deleting,
    Succeeded,
    Failed,
    Canceled,
    Deleted,
    NotSpecified,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MonitoringStatus {
    Enabled,
    Disabled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MarketplaceSubscriptionStatus {
    Provisioning,
    Active,
    Suspended,
    Unsubscribed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatadogOrganizationProperties {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "linkingAuthCode", skip_serializing_if = "Option::is_none")]
    pub linking_auth_code: Option<String>,
    #[serde(rename = "linkingClientId", skip_serializing_if = "Option::is_none")]
    pub linking_client_id: Option<String>,
    #[serde(rename = "redirectUri", skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    #[serde(rename = "apiKey", skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(rename = "applicationKey", skip_serializing_if = "Option::is_none")]
    pub application_key: Option<String>,
    #[serde(rename = "enterpriseAppId", skip_serializing_if = "Option::is_none")]
    pub enterprise_app_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "emailAddress", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(rename = "phoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LiftrResourceCategories {
    Unknown,
    MonitorLogs,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorProperties {
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "monitoringStatus", skip_serializing_if = "Option::is_none")]
    pub monitoring_status: Option<MonitoringStatus>,
    #[serde(rename = "marketplaceSubscriptionStatus", skip_serializing)]
    pub marketplace_subscription_status: Option<MarketplaceSubscriptionStatus>,
    #[serde(rename = "datadogOrganizationProperties", skip_serializing_if = "Option::is_none")]
    pub datadog_organization_properties: Option<DatadogOrganizationProperties>,
    #[serde(rename = "userInfo", skip_serializing_if = "Option::is_none")]
    pub user_info: Option<UserInfo>,
    #[serde(rename = "liftrResourceCategory", skip_serializing)]
    pub liftr_resource_category: Option<LiftrResourceCategories>,
    #[serde(rename = "liftrResourcePreference", skip_serializing)]
    pub liftr_resource_preference: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ManagedIdentityTypes {
    SystemAssigned,
    UserAssigned,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityProperties {
    #[serde(rename = "principalId", skip_serializing)]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", skip_serializing)]
    pub tenant_id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<ManagedIdentityTypes>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatadogMonitorResource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<MonitorProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub location: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatadogMonitorResourceListResponse {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DatadogMonitorResource>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorUpdateProperties {
    #[serde(rename = "monitoringStatus", skip_serializing_if = "Option::is_none")]
    pub monitoring_status: Option<MonitoringStatus>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatadogMonitorResourceUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<MonitorUpdateProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatadogSetPasswordLink {
    #[serde(rename = "setPasswordLink", skip_serializing_if = "Option::is_none")]
    pub set_password_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TagAction {
    Include,
    Exclude,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilteringTag {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<TagAction>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogRules {
    #[serde(rename = "sendAadLogs", skip_serializing_if = "Option::is_none")]
    pub send_aad_logs: Option<bool>,
    #[serde(rename = "sendSubscriptionLogs", skip_serializing_if = "Option::is_none")]
    pub send_subscription_logs: Option<bool>,
    #[serde(rename = "sendResourceLogs", skip_serializing_if = "Option::is_none")]
    pub send_resource_logs: Option<bool>,
    #[serde(rename = "filteringTags", skip_serializing_if = "Vec::is_empty")]
    pub filtering_tags: Vec<FilteringTag>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricRules {
    #[serde(rename = "filteringTags", skip_serializing_if = "Vec::is_empty")]
    pub filtering_tags: Vec<FilteringTag>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitoringTagRulesProperties {
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "logRules", skip_serializing_if = "Option::is_none")]
    pub log_rules: Option<LogRules>,
    #[serde(rename = "metricRules", skip_serializing_if = "Option::is_none")]
    pub metric_rules: Option<MetricRules>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitoringTagRules {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<MonitoringTagRulesProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitoringTagRulesListResponse {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MonitoringTagRules>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SingleSignOnStates {
    Initial,
    Enable,
    Disable,
    Existing,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatadogSingleSignOnProperties {
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "singleSignOnState", skip_serializing_if = "Option::is_none")]
    pub single_sign_on_state: Option<SingleSignOnStates>,
    #[serde(rename = "enterpriseAppId", skip_serializing_if = "Option::is_none")]
    pub enterprise_app_id: Option<String>,
    #[serde(rename = "singleSignOnUrl", skip_serializing)]
    pub single_sign_on_url: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatadogSingleSignOnResource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatadogSingleSignOnProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatadogSingleSignOnResourceListResponse {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DatadogSingleSignOnResource>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
