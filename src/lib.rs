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
        let subscriber = subscriber::Subscribers::new(
            Novu::build_backend_url(&backend_url),
            Novu::build_client(&api_key)?,
        );

        Ok(Self {
            backend_url: Novu::build_backend_url(&backend_url),
            client: Novu::build_client(&api_key)?,
            subscriber,
        })
    }

    fn build_client(api_key: &str) -> Result<reqwest::Client, Box<dyn Error>> {
        let mut default_headers1 = reqwest::header::HeaderMap::new();

        default_headers1.insert(
            "Authorization",
            reqwest::header::HeaderValue::from_str(&api_key)?,
        );

        let client = reqwest::Client::builder()
            .default_headers(default_headers1)
            .build()?;

        Ok(client)
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

        let result = self
            .client
            .post(&format!("{}/events/trigger", self.backend_url))
            .json(&data)
            .send()
            .await?;

        if result.status().is_success() {
            return Ok(());
        }

        Err(format!(
            "Failed to trigger event, API request failed with response code {}",
            result.status()
        )
        .into())
    }
}

#[cfg(test)]
#[tokio::test]
async fn test_trigger() {
    let novu = Novu::new("".to_string(), None).unwrap();
    let result = novu
        .trigger(
            "",
            ITriggerPayloadOptions {
                payload: HashMap::new(),
                to: TriggerRecipientsType::Single("".to_string()),
            },
        )
        .await;

    assert!(result.is_err());
}
