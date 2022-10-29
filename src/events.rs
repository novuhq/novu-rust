use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::IAttachmentOptions;

#[derive(Debug, Serialize, Deserialize)]
pub enum AllowedPayloadValues {
    STRING(String),
    StringArray(Vec<String>),
    BOOLEAN(bool),
    NUMBER(i32),
    UNDEFINED(()),
    AttachmentOptions(IAttachmentOptions),
    AttachmentOptionsArray(Vec<IAttachmentOptions>),
    RECORD(HashMap<String, String>),
}

pub type ITriggerPayload = HashMap<String, AllowedPayloadValues>;

#[derive(Debug, Serialize, Deserialize)]
pub enum TriggerRecipientsType {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ITriggerPayloadOptions {
    pub payload: ITriggerPayload,
    pub to: TriggerRecipientsType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerPayload {
    #[serde(rename = "eventId")]
    pub event_id: String,
    pub payload: ITriggerPayload,
    pub to: TriggerRecipientsType,
}
