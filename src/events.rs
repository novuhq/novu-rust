use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::IAttachmentOptions;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
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
#[serde(untagged)]
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
    #[serde(rename = "phone")]
    phone_number: Option<String>,
    #[serde(rename = "avatar")]
    avatar_url: Option<String>,
}

impl TriggerRecipient {
    pub fn new(subscriber_id: impl ToString) -> Self {
        Self {
            subscriber_id: subscriber_id.to_string(),
            email: None,
            first_name: None,
            last_name: None,
            phone_number: None,
            avatar_url: None,
        }
    }

    pub fn first_name(&mut self, name: impl ToString) -> &mut Self {
        self.first_name = Some(name.to_string());
        self
    }

    pub fn last_name(&mut self, name: impl ToString) -> &mut Self {
        self.last_name = Some(name.to_string());
        self
    }

    pub fn email(&mut self, email: impl ToString) -> &mut Self {
        self.email = Some(email.to_string());
        self
    }

    pub fn phone_number(&mut self, phone_number: impl ToString) -> &mut Self {
        self.phone_number = Some(phone_number.to_string());
        self
    }

    pub fn avatar_url(&mut self, avatar_url: impl ToString) -> &mut Self {
        self.avatar_url = Some(avatar_url.to_string());
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
    pub acknowledged: bool,
    pub status: String,
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
}
