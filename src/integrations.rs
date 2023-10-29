use serde::{Deserialize, Serialize};

use crate::{ChannelTypeEnum, client::{Response, Client}, error::NovuError};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Integration {
    pub _id: Option<String>,
    pub _environment_id: String,
    pub _organization_id: String,
    pub name: String,
    pub identifier: String,
    pub provider_id: String,
    pub channel: ChannelTypeEnum,
    pub credentials: Credentials,
    pub active: bool,
    pub deleted: bool,
    pub deleted_at: String,
    pub deleted_by: String,
    pub primary: bool,
    pub conditions: Option<Vec<StepFilter>>
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Credentials {
    pub api_key: Option<String>,
    pub user: Option<String>,
    pub secret_key: Option<String>,
    pub domain: Option<String>,
    pub password: Option<String>,
    pub host: Option<String>,
    pub port: Option<String>,
    pub secure: Option<bool>,
    pub region: Option<String>,
    pub account_sid: Option<String>,
    pub message_profile_id: Option<String>,
    pub token: Option<String>,
    pub from: Option<String>,
    pub sender_name: Option<String>,
    pub project_name: Option<String>,
    pub application_id: Option<String>,
    pub client_id: Option<String>,
    pub require_tls: Option<bool>,
    pub ignore_tls: Option<bool>,
    pub tls_options: Option<serde_json::Value>,
    pub base_url: Option<String>,
    pub webhook_url: Option<String>,
    pub redirect_url: Option<String>,
    pub hmac: Option<bool>,
    pub service_account: Option<String>,
    pub ip_pool_name: Option<String>
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepFilter {
    is_negated: bool,
    step_filter_type: StepFilterType,
    value: StepFilterValue,
    children: Vec<FieldFilterPart>
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StepFilterType {
    BOOLEAN,
    TEXT,
    DATE,
    NUMBER,
    STATEMENT,
    LIST,
    MultiList,
    GROUP,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StepFilterValue {
    AND,
    OR,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldFilterPart {
    pub field: String,
    pub value: String,
    pub operator: FieldFilterPartOperator,
    pub on: FieldFilterPartOn
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FieldFilterPartOperator {
    LARGER,
    SMALLER,
    LargerEqual,
    SmallerEqual,
    EQUAL,
    NotEqual,
    AllIn,
    AnyIn,
    NotIn,
    BETWEEN,
    NotBetween,
    LIKE,
    NotLike,
    IN
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FieldFilterPartOn {
    SUBSCRIBER,
    PAYLOAD
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct CreateIntegrationRequest {
    name: Option<String>,
    identifier: Option<String>,
    #[serde(rename = "_environmentId")]
    _environment_id: Option<String>,
    #[serde(rename = "providerId")]
    provider_id: String,
    channel: ChannelTypeEnum,
    credentials: Option<Credentials>,
    active: Option<bool>,
    check: Option<bool>,
    conditions: Option<Vec<StepFilter>>
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateIntegrationRequest {
    name: Option<String>,
    identifier: Option<String>,
    #[serde(rename = "_environmentId")]
    _environment_id: Option<String>,
    credentials: Option<Credentials>,
    active: Option<bool>,
    check: Option<bool>,
    conditions: Vec<StepFilter>
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelTypeLimit {
    limit: u32,
    count: u32,
}

pub struct Integrations {
    client: Client,
}
