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
#[serde(tag = "TriggerRecipientsType")]
pub enum TriggerRecipientsType {
    Single(TriggerRecipient),
    Multiple(Vec<TriggerRecipient>),
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TriggerRecipient {
    #[serde(rename = "subscriberId")]
    subscriber_id: String,
    email: Option<String>,
    #[serde(rename = "firstName")]
    first_name: Option<String>,
    #[serde(rename = "lastName")]
    last_name: Option<String>,
}

impl TriggerRecipient {
    pub fn new(subscriber_id: impl ToString) -> Self {
        Self {
            subscriber_id: subscriber_id.to_string(),
            email: None,
            first_name: None,
            last_name: None,
        }
    }

    pub fn first_name(&mut self, name: impl ToString) -> &mut Self {
        self.first_name = Some(name.to_string());
        self
    }

    pub fn build(&self) -> Self {
        self.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerPayload {
    pub payload: ITriggerPayload,
    pub to: TriggerRecipientsType,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerResponse {
    acknowledged: bool,
    status: String,
    #[serde(rename = "transactionId")]
    transaction_id: String,
}
