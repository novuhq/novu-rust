pub mod subscriber;

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};

#[derive(Debug, Serialize, Deserialize)]
pub enum ChannelTypeEnum {
    EMAIL,
    SMS,
    DIRECT,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IAttachmentOptions {
    pub mime: String,
    pub file: Vec<u8>,
    pub name: Option<String>,
    pub channels: Option<Vec<ChannelTypeEnum>>,
}

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

type ITriggerPayload = HashMap<String, AllowedPayloadValues>;

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
struct TriggerPayload {
    #[serde(rename = "eventId")]
    event_id: String,
    payload: ITriggerPayload,
    to: TriggerRecipientsType,
}

pub struct Novu {
    backend_url: String,
    client: reqwest::Client,
    pub subscriber: subscriber::Subscribers,
}

impl Novu {
    pub fn new(api_key: String, backend_url: Option<String>) -> Result<Self, Box<dyn Error>> {
        // todo: introduce a lifetime for subscriber
        let mut default_headers1 = reqwest::header::HeaderMap::new();

        default_headers1.insert(
            "Authorization",
            reqwest::header::HeaderValue::from_str(&api_key)?,
        );

        let mut default_headers2 = reqwest::header::HeaderMap::new();

        default_headers2.insert(
            "Authorization",
            reqwest::header::HeaderValue::from_str(&api_key)?,
        );

        let client1 = reqwest::Client::builder()
            .default_headers(default_headers1)
            .build()?;

        let client2 = reqwest::Client::builder()
            .default_headers(default_headers2)
            .build()?;

        Ok(Self {
            backend_url: Novu::build_backend_url(&backend_url),
            client: client1,
            subscriber: subscriber::Subscribers::new(
                Novu::build_backend_url(&backend_url),
                client2,
            ),
        })
    }

    fn build_backend_url(backend_url: &Option<String>) -> String {
        const NOVU_VERSION: &str = "v1";

        if let Some(backend_url) = backend_url {
            if backend_url.contains("novu.co/v") {
                return backend_url.to_string();
            }

            return format!("{}/{}", backend_url, NOVU_VERSION);
        }

        format!(
            "https://api.novu.co/{NOVU_VERSION}",
            NOVU_VERSION = NOVU_VERSION
        )
    }

    pub async fn trigger(
        self: &Self,
        event_id: &str,
        data: ITriggerPayloadOptions,
    ) -> Result<(), Box<dyn Error>> {
        let data = TriggerPayload {
            event_id: event_id.to_string(),
            payload: data.payload,
            to: data.to,
        };

        self.client
            .post(&format!("{}/events/trigger", self.backend_url))
            .json(&data)
            .send()
            .await?;

        Ok(())
    }
}
