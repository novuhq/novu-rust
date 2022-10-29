pub mod events;
pub mod subscriber;

use events::{ITriggerPayloadOptions, TriggerPayload};
use serde::{Deserialize, Serialize};
use std::error::Error;

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

pub struct Novu {
    backend_url: String,
    client: reqwest::Client,
    pub subscriber: subscriber::Subscribers,
}

impl Novu {
    // This X generic allows to pass &str or String
    pub fn new<X: ToString>(api_key: X, backend_url: Option<X>) -> Result<Self, Box<dyn Error>> {
        let subscriber = subscriber::Subscribers::new(
            Self::build_backend_url(&backend_url),
            Self::build_client(&api_key)?,
        );

        Ok(Self {
            backend_url: Self::build_backend_url(&backend_url),
            client: Self::build_client(&api_key)?,
            subscriber,
        })
    }

    // This ApiKey generic allows to pass &str or String
    fn build_client<ApiKey: ToString>(api_key: &ApiKey) -> Result<reqwest::Client, Box<dyn Error>> {
        let mut default_headers = reqwest::header::HeaderMap::new();

        default_headers.insert(
            "Authorization",
            reqwest::header::HeaderValue::from_str(&api_key.to_string())?,
        );

        let client = reqwest::Client::builder()
            .default_headers(default_headers)
            .build()?;

        Ok(client)
    }

    // This ApiUrl generic allows to pass &str or String
    fn build_backend_url<ApiUrl: ToString>(backend_url: &Option<ApiUrl>) -> String {
        const NOVU_VERSION: &str = "v1";

        if let Some(backend_url) = backend_url {
            let backend_url = &backend_url.to_string();

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
        self,
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
    let novu = Novu::new("", None).unwrap();
    let result = novu
        .trigger(
            "",
            ITriggerPayloadOptions {
                payload: std::collections::HashMap::new(),
                to: events::TriggerRecipientsType::Single("".to_string()),
            },
        )
        .await;

    assert!(result.is_err());
}
